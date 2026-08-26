#!/usr/bin/env python3
"""
FQM 三维进制解释器 · 声母×韵母×声调 = 三维空间坐标

核心概念：
- 每个符号在三维空间中有唯一坐标 (声母位置, 韵母位置, 声调位置)
- 坐标用本体论路径表示：(声母.首, 韵母.首, 声调.首)
- 支持三维空间中的距离计算、邻近查询、路径查找

操作语法：
  三维.坐标(b, a, 1)     → 查询符号b/a/1的三维坐标
  三维.距离(b, a, 1, p, o, 2)  → 计算两个坐标点的距离
  三维.邻近(b, a, 1)     → 查询邻近坐标的符号
  三维.路径(b, a, 1, p, o, 2)  → 查找从一点到另一点的路径
"""

import os
import sys
import re
import math


class Point3D:
    """三维坐标点 - 用本体论路径表示"""
    
    def __init__(self, shengmu_identity, yunmu_identity, tone_identity):
        self.shengmu = shengmu_identity  # 声母身份: 首/次/三...
        self.yunmu = yunmu_identity      # 韵母身份
        self.tone = tone_identity        # 声调身份
    
    def __str__(self):
        return f"(声母.{self.shengmu}, 韵母.{self.yunmu}, 声调.{self.tone})"
    
    def __repr__(self):
        return f"Point3D({self.shengmu}, {self.yunmu}, {self.tone})"
    
    def __eq__(self, other):
        return self.shengmu == other.shengmu and self.yunmu == other.yunmu and self.tone == other.tone
    
    def __hash__(self):
        return hash((self.shengmu, self.yunmu, self.tone))
    
    def to_dict(self):
        return {
            "声母": self.shengmu,
            "韵母": self.yunmu,
            "声调": self.tone
        }


class Radix3DInterpreter:
    """三维进制解释器"""
    
    def __init__(self, linker):
        self.linker = linker
        self.coordinates = {}    # Point3D -> (shengmu_sym, yunmu_sym, tone_sym)
        self._build_coordinates()
    
    def _build_coordinates(self):
        """构建所有有效的三维坐标"""
        # 获取所有容器
        shengmu_container = self.linker.loader.get_container("声母")
        yunmu_container = self.linker.loader.get_container("韵母")
        tone_container = self.linker.loader.get_container("声调")
        
        if not shengmu_container or not yunmu_container or not tone_container:
            return
        
        # 为每个已链接的声母-韵母对创建坐标
        for link in self.linker.resolved:
            from_sym = link['from']['symbol']
            to_sym = link['to']['symbol']
            from_path = link['from']['path']
            to_path = link['to']['path']
            
            # 为每个声调创建坐标点
            for tone_entry in tone_container.symbols:
                tone_sym = tone_entry["symbol"]
                # 查找声调的身份
                tone_identity = self._find_identity("声调", tone_sym)
                if tone_identity:
                    point = Point3D(from_path.identity, to_path.identity, tone_identity)
                    self.coordinates[point] = (from_sym, to_sym, tone_sym)
    
    def _find_identity(self, domain, symbol):
        """查找符号在容器中的身份"""
        for (d, s), p in self.linker.symbol_to_path.items():
            if d == domain and s == symbol:
                return p.identity
        return None
    
    def _find_symbol(self, domain, identity):
        """查找身份对应的符号"""
        for (d, s), p in self.linker.symbol_to_path.items():
            if d == domain and p.identity == identity:
                return s
        return None
    
    def execute(self, command):
        """执行三维命令"""
        command = command.strip()
        if not command or command.startswith('#'):
            return None
        
        # 三维.坐标(b, a, 1)
        if command.startswith('三维.坐标('):
            return self._cmd_coordinate(command)
        
        # 三维.距离(b, a, 1, p, o, 2)
        if command.startswith('三维.距离('):
            return self._cmd_distance(command)
        
        # 三维.邻近(b, a, 1)
        if command.startswith('三维.邻近('):
            return self._cmd_neighbors(command)
        
        # 三维.路径(b, a, 1, p, o, 2)
        if command.startswith('三维.路径('):
            return self._cmd_path(command)
        
        # 三维.空间 - 列出所有坐标
        if command == '三维.空间':
            return self._cmd_space()
        
        return f"错误: 未知命令 {command}"
    
    def _parse_triplet(self, s):
        """解析 (b, a, 1) 这样的三元组"""
        match = re.match(r'\(([^,]+),\s*([^,]+),\s*([^)]+)\)', s)
        if not match:
            return None
        return match.group(1).strip(), match.group(2).strip(), match.group(3).strip()
    
    def _symbol_to_identity(self, domain, symbol):
        """符号转身份"""
        return self._find_identity(domain, symbol)
    
    def _cmd_coordinate(self, command):
        """查询坐标: 三维.坐标(b, a, 1)"""
        match = re.match(r'三维\.坐标\((.+)\)', command)
        if not match:
            return "错误: 坐标格式"
        
        parts = self._parse_triplet(f"({match.group(1)})")
        if not parts:
            return "错误: 无法解析坐标参数"
        
        shengmu_sym, yunmu_sym, tone_sym = parts
        
        # 查找各维度的身份
        sm_id = self._symbol_to_identity("声母", shengmu_sym)
        ym_id = self._symbol_to_identity("韵母", yunmu_sym)
        t_id = self._symbol_to_identity("声调", tone_sym)
        
        if not sm_id:
            return f"错误: 声母中无 {shengmu_sym}"
        if not ym_id:
            return f"错误: 韵母中无 {yunmu_sym}"
        if not t_id:
            return f"错误: 声调中无 {tone_sym}"
        
        point = Point3D(sm_id, ym_id, t_id)
        
        # 查找该坐标的符号
        if point in self.coordinates:
            syms = self.coordinates[point]
            result = f"坐标 {point} = (声母.{syms[0]}, 韵母.{syms[1]}, 声调.{syms[2]})"
        else:
            result = f"坐标 {point} = 空（无符号占据）"
        
        return result
    
    def _cmd_distance(self, command):
        """计算距离: 三维.距离(b, a, 1, p, o, 2)"""
        # 提取括号内的所有内容
        match = re.match(r'三维\.距离\((.+)\)', command)
        if not match:
            return "错误: 距离格式"
        
        # 按逗号分割成6个参数
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 6:
            return "错误: 距离需要6个参数 (b, a, 1, p, o, 2)"
        
        p1 = (args[0], args[1], args[2])
        p2 = (args[3], args[4], args[5])
        
        if not p1 or not p2:
            return "错误: 无法解析距离参数"
        
        # 转换为身份
        sm1 = self._symbol_to_identity("声母", p1[0])
        ym1 = self._symbol_to_identity("韵母", p1[1])
        t1 = self._symbol_to_identity("声调", p1[2])
        
        sm2 = self._symbol_to_identity("声母", p2[0])
        ym2 = self._symbol_to_identity("韵母", p2[1])
        t2 = self._symbol_to_identity("声调", p2[2])
        
        if not all([sm1, ym1, t1, sm2, ym2, t2]):
            return "错误: 坐标参数中有未定义符号"
        
        # 计算本体论距离（身份差距之和）
        dist = self._ontology_distance(sm1, sm2) + self._ontology_distance(ym1, ym2) + self._ontology_distance(t1, t2)
        
        result = f"距离: ({p1[0]},{p1[1]},{p1[2]}) → ({p2[0]},{p2[1]},{p2[2]}) = {dist}"
        return result
    
    def _ontology_distance(self, id1, id2):
        """计算两个身份之间的距离"""
        identity_order = ["首", "次", "三", "四", "五", "六", "七", "八", "九", "十",
                         "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
                         "二十一", "二十二", "二十三", "二十四"]
        
        if id1 in identity_order and id2 in identity_order:
            return abs(identity_order.index(id1) - identity_order.index(id2))
        return 0
    
    def _cmd_neighbors(self, command):
        """查询邻近: 三维.邻近(b, a, 1)"""
        match = re.match(r'三维\.邻近\((.+)\)', command)
        if not match:
            return "错误: 邻近格式"
        
        parts = self._parse_triplet(f"({match.group(1)})")
        if not parts:
            return "错误: 无法解析邻近参数"
        
        shengmu_sym, yunmu_sym, tone_sym = parts
        
        sm_id = self._symbol_to_identity("声母", shengmu_sym)
        ym_id = self._symbol_to_identity("韵母", yunmu_sym)
        t_id = self._symbol_to_identity("声调", tone_sym)
        
        if not all([sm_id, ym_id, t_id]):
            return "错误: 坐标参数中有未定义符号"
        
        # 查找邻近点（距离为1的点）
        neighbors = []
        identity_order = ["首", "次", "三", "四", "五", "六", "七", "八", "九", "十",
                         "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
                         "二十一", "二十二", "二十三", "二十四"]
        
        # 生成邻近身份
        nearby_identities = []
        for domain_id, domain_name in [(sm_id, "声母"), (ym_id, "韵母"), (t_id, "声调")]:
            if domain_id in identity_order:
                idx = identity_order.index(domain_id)
                if idx > 0:
                    nearby_identities.append((domain_name, identity_order[idx-1]))
                if idx < len(identity_order) - 1:
                    nearby_identities.append((domain_name, identity_order[idx+1]))
        
        result = f"邻近 ({shengmu_sym},{yunmu_sym},{tone_sym}):\n"
        for domain, identity in nearby_identities:
            sym = self._find_symbol(domain, identity)
            if sym:
                result += f"  {domain}.{identity}({sym})\n"
        
        return result.strip()
    
    def _cmd_path(self, command):
        """查找路径: 三维.路径(b, a, 1, p, o, 2)"""
        match = re.match(r'三维\.路径\((.+)\)', command)
        if not match:
            return "错误: 路径格式"
        
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 6:
            return "错误: 路径需要6个参数 (b, a, 1, p, o, 2)"
        
        p1 = (args[0], args[1], args[2])
        p2 = (args[3], args[4], args[5])
        
        if not p1 or not p2:
            return "错误: 无法解析路径参数"
        
        # 简单路径：沿各维度逐步移动
        sm1 = self._symbol_to_identity("声母", p1[0])
        ym1 = self._symbol_to_identity("韵母", p1[1])
        t1 = self._symbol_to_identity("声调", p1[2])
        
        sm2 = self._symbol_to_identity("声母", p2[0])
        ym2 = self._symbol_to_identity("韵母", p2[1])
        t2 = self._symbol_to_identity("声调", p2[2])
        
        if not all([sm1, ym1, t1, sm2, ym2, t2]):
            return "错误: 路径参数中有未定义符号"
        
        identity_order = ["首", "次", "三", "四", "五", "六", "七", "八", "九", "十",
                         "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
                         "二十一", "二十二", "二十三", "二十四"]
        
        path = []
        # 沿声母维度移动
        if sm1 in identity_order and sm2 in identity_order:
            idx1 = identity_order.index(sm1)
            idx2 = identity_order.index(sm2)
            step = 1 if idx2 > idx1 else -1
            for idx in range(idx1, idx2, step):
                path.append(f"声母.{identity_order[idx]} → 声母.{identity_order[idx+step]}")
        
        # 沿韵母维度移动
        if ym1 in identity_order and ym2 in identity_order:
            idx1 = identity_order.index(ym1)
            idx2 = identity_order.index(ym2)
            step = 1 if idx2 > idx1 else -1
            for idx in range(idx1, idx2, step):
                path.append(f"韵母.{identity_order[idx]} → 韵母.{identity_order[idx+step]}")
        
        # 沿声调维度移动
        if t1 in identity_order and t2 in identity_order:
            idx1 = identity_order.index(t1)
            idx2 = identity_order.index(t2)
            step = 1 if idx2 > idx1 else -1
            for idx in range(idx1, idx2, step):
                path.append(f"声调.{identity_order[idx]} → 声调.{identity_order[idx+step]}")
        
        if path:
            result = f"路径 ({p1[0]},{p1[1]},{p1[2]}) → ({p2[0]},{p2[1]},{p2[2]}):\n"
            result += "\n".join(f"  {i+1}. {p}" for i, p in enumerate(path))
        else:
            result = "已在同一点"
        
        return result
    
    def _cmd_space(self):
        """列出所有坐标"""
        if not self.coordinates:
            return "三维空间为空"
        
        result = "三维空间坐标:\n"
        for point, syms in self.coordinates.items():
            result += f"  {point} → {syms}\n"
        
        return result.strip()
    
    def display_space(self):
        """显示三维空间"""
        print(f"\n{'='*60}")
        print("三维进制空间")
        print(f"{'-'*60}")
        
        if not self.coordinates:
            print("  空间为空")
        else:
            for point, syms in self.coordinates.items():
                print(f"  {point}")
                print(f"    → 声母.{syms[0]}, 韵母.{syms[1]}, 声调.{syms[2]}")
        
        print(f"{'='*60}\n")


# ============================================================
# 完整管道（含三维解释器）
# ============================================================

class FQMPipeline:
    """FQM 完整管道 v3（含三维解释器）"""
    
    def __init__(self, fqm_file):
        self.fqm_file = fqm_file
        self.loader = None
        self.linker = None
        self.interpreter = None
        self.radix3d = None
    
    def run(self):
        """运行完整管道"""
        from loader import FQMLoader
        from linker import FQMLinker
        from interpreter import FQMInterpreter
        
        self.loader = FQMLoader()
        if not self.loader.load_file(self.fqm_file):
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.interpreter = FQMInterpreter(self.linker)
        self.radix3d = Radix3DInterpreter(self.linker)
        
        return True
    
    def execute(self, command):
        """执行命令"""
        # 判断是三维命令还是普通命令
        if command.strip().startswith('三维.'):
            return self.radix3d.execute(command)
        return self.interpreter.execute(command)
    
    def display_all(self):
        """显示全部"""
        self.loader.display_summary()
        self.linker.display_ontology()
        self.linker.display_resolution()
        self.interpreter.display_memory()
        self.radix3d.display_space()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("=" * 60)
    print("FQM 三维进制解释器测试")
    print("=" * 60)
    
    fqm_file = "containers.fqm"
    if not os.path.exists(fqm_file):
        print(f"创建测试文件: {fqm_file}")
        with open(fqm_file, 'w') as f:
            f.write("""# 测试容器
袋.声母.容=b,p,m,f
袋.声母.进制=4
袋.声母.极性=1

袋.韵母.容=a,o,e,i
袋.韵母.进制=4
袋.韵母.极性=0

袋.声调.容=1,2,3,4
袋.声调.进制=4
袋.声调.极性=1

袋.声母.引.韵母=b→a,p→o
""")
    
    pipe = FQMPipeline(fqm_file)
    if not pipe.run():
        print("管道启动失败")
        sys.exit(1)
    
    pipe.display_all()
    
    print("\n三维命令测试:")
    commands = [
        "三维.空间",
        "三维.坐标(b, a, 1)",
        "三维.距离(b, a, 1, p, o, 2)",
        "三维.邻近(b, a, 1)",
        "三维.路径(b, a, 1, p, o, 2)",
    ]
    
    for cmd in commands:
        result = pipe.execute(cmd)
        if result:
            print(f"\n  {cmd}")
            for line in result.split('\n'):
                print(f"    {line}")
