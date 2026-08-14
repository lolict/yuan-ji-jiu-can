#!/usr/bin/env python3
"""
UniCore 通用数学序列生成器
================================
探索各种数学序列的规律和应用！
"""

import sys
from typing import List, Callable, Dict, Tuple
from dataclasses import dataclass, field
from collections import defaultdict
import json


@dataclass
class SequencePattern:
    """序列模式"""
    name: str
    description: str
    operation: str  # '+', '*', '-', '/'
    k: int          # 前面 k 个数参与
    coefficients: List[int] = None  # 系数，比如 [1,1] 就是 F(n) = F(n-1)+F(n-2)
    examples: List[int] = None
    applications: List[str] = None
    domain: str = "整数"


@dataclass
class GeneratedSequence:
    """生成的序列"""
    name: str
    sequence: List[int]
    pattern: SequencePattern
    metadata: Dict = field(default_factory=dict)


class UniversalSequenceGenerator:
    """通用序列生成器"""
    
    def __init__(self):
        self.patterns = []
        self._register_basic_patterns()
    
    def _register_basic_patterns(self):
        """注册基础模式"""
        self.patterns.append(
            SequencePattern(
                name="斐波那契序列 (k=2, 加法)",
                description="F(n) = F(n-1) + F(n-2)",
                operation="+",
                k=2,
                coefficients=[1, 1],
                examples=[0, 1, 1, 2, 3, 5, 8, 13, 21, 34],
                applications=[
                    "花瓣排列（向日葵、雏菊）",
                    "叶序排列",
                    "金融市场分析",
                    "计算机科学（斐波那契堆）"
                ]
            )
        )
        
        # k=3 加法
        self.patterns.append(
            SequencePattern(
                name="三角波序列 (k=3, 加法)",
                description="T(n) = T(n-1) + T(n-2) + T(n-3)",
                operation="+",
                k=3,
                coefficients=[1, 1, 1],
                examples=[0, 1, 1, 2, 4, 7, 13, 24, 44, 81],
                applications=[
                    "物理振动分析",
                    "信号处理"
                ]
            )
        )
        
        # k=2 乘法
        self.patterns.append(
            SequencePattern(
                name="乘法序列 (k=2, 乘法)",
                description="M(n) = M(n-1) * M(n-2)",
                operation="*",
                k=2,
                coefficients=[1, 1],
                examples=[2, 3, 6, 18, 108, 1944],
                applications=[
                    "指数增长模型",
                    "复合利息计算"
                ]
            )
        )
        
        # k=2 交替加法
        self.patterns.append(
            SequencePattern(
                name="交替序列 (k=2, 交替系数)",
                description="A(n) = 2*A(n-1) + 3*A(n-2)",
                operation="+",
                k=2,
                coefficients=[2, 3],
                examples=[1, 1, 5, 13, 41, 121, 365],
                applications=["递归算法"]
            )
        )
        
        # k=3 平均
        self.patterns.append(
            SequencePattern(
                name="移动平均序列 (k=3, 除以3)",
                description="Avg(n) = (Avg(n-1) + Avg(n-2) + Avg(n-3)) // 3",
                operation="+",
                k=3,
                coefficients=[1, 1, 1],
                examples=[100, 200, 300, 200, 233, 244, 225, 234, 234, 231],
                applications=[
                    "数据平滑",
                    "噪声过滤"
                ]
            )
        )
        
        # 自定义混合
        self.patterns.append(
            SequencePattern(
                name="混合序列 (加法+乘法)",
                description="Hybrid(n) = 2*H(n-1) + H(n-2)*H(n-3)",
                operation="+*",
                k=3,
                coefficients=[2, 1, 1],
                examples=[1, 2, 3, 7, 20, 61, 182],
                applications=["非线性系统研究"]
            )
        )
    
    def generate(self, pattern: SequencePattern, length: int = 30) -> GeneratedSequence:
        """生成指定长度的序列"""
        seq = []
        
        # 初始条件
        if pattern.name.startswith("斐波那契"):
            seq = [0, 1]
        elif pattern.name.startswith("三角波"):
            seq = [0, 1, 1]
        elif pattern.name.startswith("乘法"):
            seq = [2, 3]
        elif pattern.name.startswith("交替"):
            seq = [1, 1]
        elif pattern.name.startswith("移动平均"):
            seq = [100, 200, 300]
        else:
            seq = [1, 2, 3]
        
        # 补充到 k 个初始值
        while len(seq) < pattern.k:
            seq.append(seq[-1] + 1)
        
        # 生成
        for i in range(len(seq), length):
            next_val = 0
            
            if pattern.operation == "+":
                for j in range(pattern.k):
                    coeff = pattern.coefficients[j] if pattern.coefficients else 1
                    next_val += coeff * seq[i - j - 1]
            elif pattern.operation == "-":
                next_val = seq[i-1] - seq[i-2]
            elif pattern.operation == "*":
                if pattern.k == 2:
                    next_val = seq[i-1] * seq[i-2]
                else:
                    next_val = seq[i-1] * seq[i-2] * seq[i-3]
            elif pattern.operation == "+*":
                if pattern.k == 3:
                    next_val = 2 * seq[i-1] + seq[i-2] * seq[i-3]
            
            # 处理除法（仅整数除法）
            if pattern.operation == "/" and pattern.k == 2 and seq[i-2] != 0:
                next_val = seq[i-1] // seq[i-2]
            
            seq.append(next_val)
        
        return GeneratedSequence(
            name=pattern.name,
            sequence=seq,
            pattern=pattern
        )
    
    def generate_custom(self, k: int, operation: str, initial: List[int], length: int = 30) -> GeneratedSequence:
        """自定义序列生成"""
        pattern = SequencePattern(
            name=f"自定义序列 (k={k}, {operation})",
            description=f"自定义模式",
            operation=operation,
            k=k,
            examples=initial
        )
        
        seq = initial.copy()
        
        for i in range(len(seq), length):
            next_val = 0
            
            if operation == "+":
                next_val = sum(seq[i-j-1] for j in range(k))
            elif operation == "*":
                next_val = 1
                for j in range(k):
                    next_val *= seq[i-j-1]
            elif operation == "alternate":
                # 交替加法
                next_val = seq[i-1] + (seq[i-2] if k >=2 else 0)
                if k >=3:
                    next_val += seq[i-3]
            
            seq.append(next_val)
        
        return GeneratedSequence(
            name=f"自定义序列 (k={k}, {operation})",
            sequence=seq,
            pattern=pattern
        )
    
    def find_patterns(self, sequence: List[int]) -> List[Tuple[str, float]]:
        """分析序列，寻找规律"""
        candidates = []
        
        # 检查斐波那契类
        if len(sequence) > 4:
            if (sequence[3] == sequence[2] + sequence[1] and 
                sequence[4] == sequence[3] + sequence[2]):
                candidates.append(("斐波那契 (k=2)", 1.0))
        
        # 检查加法序列
        if len(sequence) > 5:
            if (sequence[4] == sequence[3] + sequence[2] + sequence[1] and
                sequence[5] == sequence[4] + sequence[3] + sequence[2]):
                candidates.append(("三角波 (k=3)", 1.0))
        
        return candidates
    
    def print_all_patterns(self):
        """打印所有模式"""
        print("\n" + "="*80)
        print("📚 UniCore 通用序列生成器 - 可用模式")
        print("="*80)
        
        for i, pattern in enumerate(self.patterns):
            print(f"\n{i+1}. {pattern.name}")
            print(f"   公式: {pattern.description}")
            if pattern.applications:
                print(f"   应用:")
                for app in pattern.applications:
                    print(f"      • {app}")
            if pattern.examples:
                print(f"   例子: {pattern.examples[:10]}")


# 演示
def demo():
    print("""
╔═══════════════════════════════════════════════════════════════════╗
║                 UniCore 通用数学序列生成器                     ║
╚═══════════════════════════════════════════════════════════════════╝
    """)
    
    generator = UniversalSequenceGenerator()
    generator.print_all_patterns()
    
    print("\n" + "="*80)
    print("🚀 生成示例序列")
    print("="*80)
    
    for pattern in generator.patterns[:4]:
        generated = generator.generate(pattern, 20)
        print(f"\n🎯 {generated.name}:")
        print(f"   {generated.sequence[:12]}")
    
    print("\n" + "="*80)
    print("💡 数学规律的实际应用")
    print("="*80)
    
    print("""
为什么这些序列重要？

1. 🌿 **自然界**：
   - 斐波那契 - 花瓣排列、叶序
   - 三角波 - 声波振动模式

2. 💻 **计算机科学**：
   - 递归算法
   - 数据结构（堆、树）
   - 密码学（序列生成）

3. 📈 **金融与预测**：
   - 市场趋势分析
   - 复合增长模型

4. 🔬 **科学与工程**：
   - 流体动力学
   - 振动系统
   - 信号处理

5. 🎨 **艺术与设计**：
   - 分形几何
   - 图案生成
   - 黄金比例应用
    """)
    
    print("\n🎨 自定义序列生成")
    print("比如: 前面4个数相加")
    custom = generator.generate_custom(k=4, operation="+", initial=[1,1,1,1], length=15)
    print(f"   {custom.sequence}")


# 交互式模式
def interactive():
    print("\n" + "="*80)
    print("🔧 交互式序列生成器")
    print("="*80)
    
    generator = UniversalSequenceGenerator()
    
    print("\n请选择:")
    print("1. 使用内置模式")
    print("2. 自定义创建序列")
    print("3. 分析现有序列")
    
    choice = input("\n选择 (1/2/3): ").strip()
    
    if choice == "1":
        generator.print_all_patterns()
        idx = int(input("\n选择模式编号: ")) - 1
        
        length = int(input("生成长度 (默认20): ") or 20)
        
        if 0 <= idx < len(generator.patterns):
            generated = generator.generate(generator.patterns[idx], length)
            print(f"\n✅ 生成成功!")
            print(f"{generated.sequence}")
    
    elif choice == "2":
        k = int(input("前面几个数参与? (k=2,3,4,...): "))
        op = input("运算方式? (+, *, -, /): ")
        init = input(f"输入 {k} 个初始值，空格分隔: ").split()
        initial = [int(x) for x in init[:k]]
        
        generated = generator.generate_custom(k, op, initial, length=25)
        print(f"\n✅ 生成:")
        print(f"{generated.sequence}")
    
    elif choice == "3":
        seq = input("输入序列，空格分隔: ").split()
        seq = [int(x) for x in seq]
        
        patterns = generator.find_patterns(seq)
        print("\n🔍 分析结果:")
        for name, conf in patterns:
            print(f"  {name} (置信度: {conf*100}%)")


if __name__ == "__main__":
    if "--interactive" in sys.argv:
        interactive()
    else:
        demo()
