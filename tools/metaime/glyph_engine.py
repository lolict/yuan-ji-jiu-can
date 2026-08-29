#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
字形几何编程语言 · 解析器 + 执行引擎

解析: 音节序列 → 抽象语法树 (AST)
执行: AST → 力场演化 → 最终状态
"""

import re
import math
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass, field

# ============================================================
# 数据类型定义
# ============================================================

@dataclass
class ForceElement:
    """几何基元"""
    geo_type: str     # straight/curve/cross/bend/closed/open/spiral/point
    zone: str         # top/bottom/middle/inside/outside/junction/endpoint/inflection/spiral_boundary/fine_transition
    desc: str
    
    @property
    def force_value(self) -> int:
        """力值"""
        return {
            'straight': 2,   # 张力
            'curve': 3,      # 扭力
            'cross': 4,      # 分支
            'bend': 2,       # 反射
            'closed': 5,     # 容器
            'open': 1,       # 通道
            'spiral': 6,     # 内外共存
            'point': 1,      # 标记
        }.get(self.geo_type, 1)


@dataclass
class GlyphInfo:
    """字母几何信息"""
    symbol: str
    primitives: List[ForceElement]
    operation: str
    zone_used: str
    geo_type: str  # sphere/line/peak/spiral/cross/frame
    
    @property
    def force_strength(self) -> int:
        """力场强度 = 所有基元力之和"""
        return sum(p.force_value for p in self.primitives)


@dataclass
class Syllable:
    """音节 = 声母 + 韵母 + 声调"""
    shengmu: str
    yunmu: str
    tone: int
    
    @property
    def pinyin(self) -> str:
        tones = {1: '', 2: '', 3: '', 4: ''}
        return f"{self.shengmu}{self.yunmu}"


@dataclass
class FieldState:
    """力场状态"""
    syllables: List[Dict] = field(default_factory=list)
    equilibrium: bool = False
    iterations: int = 0
    
    def add_syllable(self, s: Dict, pos: int):
        self.syllables.append({
            'syllable': s,
            'position': pos,
            'force': 0,
            'energy': 0,
            'active': True,
        })


# ============================================================
# 字形注册表
# ============================================================

GLYPH_REGISTRY: Dict[str, GlyphInfo] = {}

def _register(symbol: str, primitives: List[Tuple[str, str, str]], 
              operation: str, zone_used: str, geo_type: str):
    """注册字母"""
    GLYPH_REGISTRY[symbol] = GlyphInfo(
        symbol=symbol,
        primitives=[ForceElement(gt, z, d) for gt, z, d in primitives],
        operation=operation,
        zone_used=zone_used,
        geo_type=geo_type,
    )

# --- 声母 ---
_register('b', [('straight', 'bottom', '直线'), ('curve', 'middle', '曲线'),
               ('cross', 'junction', '交界'), ('closed', 'inside', '封闭')],
         '装入: 直线支撑曲线，中间交界被利用', 'middle', 'sphere')
_register('p', [('straight', 'top', '直线'), ('curve', 'bottom', '曲线'),
               ('cross', 'junction', '交界'), ('closed', 'inside', '封闭')],
         '取出: 直线悬挂曲线，中间交界被利用', 'middle', 'sphere')
_register('m', [('curve', 'top', '峰1'), ('curve', 'middle', '谷'),
               ('curve', 'top', '峰2'), ('straight', 'bottom', '基线')],
         '遍历: 三个容器并排，峰谷交替被利用', 'middle', 'peak')
_register('f', [('straight', 'top', '上横'), ('straight', 'middle', '竖'),
               ('straight', 'bottom', '下横'), ('bend', 'bottom', '弯钩')],
         '摩擦: 三向直线，接触被利用', 'bottom', 'cross')
_register('d', [('curve', 'middle', '曲线'), ('straight', 'middle', '直线'),
               ('point', 'endpoint', '共同起点'), ('point', 'endpoint', '共同终点')],
         '定位: 曲线直线共享起点终点', 'endpoint', 'sphere')
_register('t', [('straight', 'top', '横'), ('straight', 'middle', '竖'),
               ('cross', 'junction', '交叉'), ('bend', 'bottom', '钩')],
         '锚定: 交叉点固定位置，弯钩锁定', 'junction', 'cross')
_register('n', [('curve', 'top', '峰'), ('curve', 'bottom', '谷'),
               ('straight', 'middle', '左支撑'), ('straight', 'middle', '右支撑')],
         '比较: 框架结构，峰谷过渡被利用', 'junction', 'frame')
_register('l', [('straight', 'middle', '全直线'), ('open', 'top', '上开口'),
               ('open', 'bottom', '下开口'), ('point', 'endpoint', '端点')],
         '连接: 纯直线，两端开口，全部被利用', 'middle', 'line')
_register('g', [('curve', 'inside', '内螺旋'), ('spiral', 'spiral_boundary', '螺旋边界'),
               ('curve', 'outside', '外螺旋'), ('bend', 'bottom', '弯钩')],
         '螺旋: 曲线同时产生向内和向外螺旋', 'spiral_boundary', 'spiral')
_register('k', [('straight', 'middle', '左斜'), ('straight', 'middle', '右斜'),
               ('straight', 'middle', '竖'), ('cross', 'junction', '分叉点')],
         '支撑: 从一点向外分叉，扩散力', 'junction', 'cross')
_register('h', [('straight', 'middle', '左竖'), ('curve', 'top', '右弯'),
               ('straight', 'middle', '右竖'), ('bend', 'junction', '弯折')],
         '承载: 椅子形状，弯折处承载上方', 'junction', 'frame')
_register('j', [('straight', 'top', '直线'), ('curve', 'bottom', '曲线'),
               ('point', 'top', '标记点'), ('bend', 'fine_transition', '纤细过渡')],
         '捕获: 直线略多于曲线，只有纤细过渡', 'fine_transition', 'spiral')
_register('q', [('straight', 'top', '直线'), ('curve', 'bottom', '曲线'),
               ('bend', 'fine_transition', '纤细过渡'), ('curve', 'inside', '内部曲线')],
         '追踪: 曲线自我包围，内外进出被利用', 'inside', 'spiral')
_register('x', [('straight', 'top', '左上-右下'), ('straight', 'top', '右上-左下'),
               ('cross', 'junction', '交叉点'), ('open', 'endpoint', '四端开口')],
         '比较: 两线交叉，找交集', 'junction', 'cross')
_register('zh', [('straight', 'top', '横'), ('straight', 'middle', '折'),
                ('straight', 'bottom', '横'), ('bend', 'junction', '折转')],
         '折叠: 折线改变方向', 'junction', 'line')
_register('ch', [('curve', 'middle', '弯'), ('straight', 'middle', '竖'),
                ('closed', 'middle', '环抱'), ('bend', 'junction', '弯折')],
         '包裹: 环抱+支撑，环抱内容', 'middle', 'sphere')
_register('sh', [('curve', 'top', '上弯'), ('curve', 'middle', '中转折'),
                ('curve', 'bottom', '下弯'), ('bend', 'inflection', '拐点')],
         '流动: 纯曲线弯转，传递力', 'inflection', 'peak')
_register('r', [('straight', 'top', '上竖'), ('curve', 'top', '右弯'),
               ('bend', 'junction', '弯折'), ('open', 'bottom', '下开口')],
         '分支: 直线+短曲线，弯折处被利用', 'junction', 'peak')
_register('z', [('straight', 'top', '上横'), ('straight', 'middle', '斜'),
               ('straight', 'bottom', '下横'), ('bend', 'junction', '折转')],
         '振荡: 锯齿形折转，来回切换', 'junction', 'line')
_register('c', [('curve', 'middle', '弯月'), ('open', 'middle', '右开口'),
               ('closed', 'middle', '左封闭'), ('curve', 'middle', '曲率')],
         '包围: 半包围，开口向右', 'middle', 'sphere')
_register('s', [('curve', 'top', '上弯'), ('curve', 'inflection', '中转折'),
               ('curve', 'bottom', '下弯'), ('bend', 'inflection', '拐点')],
         '传递: 纯曲线弯转，中间拐点被利用', 'inflection', 'peak')
_register('y', [('straight', 'top', '左上'), ('straight', 'top', '右上'),
               ('cross', 'junction', '分叉'), ('open', 'bottom', '下开口')],
         '分裂: 一分为二，分叉处被利用', 'junction', 'peak')
_register('w', [('straight', 'top', '左上'), ('straight', 'bottom', '左下谷'),
               ('straight', 'top', '右上'), ('straight', 'bottom', '右下谷')],
         '合并: 两低一高，分叉再汇聚', 'bottom', 'peak')

# --- 韵母向量 ---
YUNMU_VECTORS = {
    'a': {'direction': 'radial_out', 'force': '发散力', 'vector': (0, 1)},
    'o': {'direction': 'circular', 'force': '扭力/向心力', 'vector': (1, 0)},
    'e': {'direction': 'diagonal', 'force': '剪切力', 'vector': (1, 1)},
    'i': {'direction': 'linear', 'force': '张力/拉力', 'vector': (0, 1)},
    'u': {'direction': 'pipe', 'force': '压力/推力', 'vector': (1, 0)},
    'ü': {'direction': 'narrow', 'force': '聚焦力', 'vector': (0.5, 0.5)},
    'ai': {'direction': 'radial_to_linear', 'force': '发散→拉力', 'vector': (0, 1.2)},
    'ei': {'direction': 'diagonal_to_linear', 'force': '剪切→拉力', 'vector': (0.5, 1)},
    'ao': {'direction': 'radial_to_circular', 'force': '发散→扭力', 'vector': (1.2, 1)},
    'ou': {'direction': 'circular_to_pipe', 'force': '扭力→压力', 'vector': (1, 0.8)},
    'an': {'direction': 'radial_to_frame', 'force': '发散→比较', 'vector': (0.5, 1.2)},
    'en': {'direction': 'diagonal_to_frame', 'force': '剪切→比较', 'vector': (0.8, 1)},
    'ang': {'direction': 'radial_to_deep', 'force': '发散→承载', 'vector': (0.3, 1.5)},
    'eng': {'direction': 'diagonal_to_deep', 'force': '剪切→承载', 'vector': (0.5, 1.3)},
    'ong': {'direction': 'circular_to_deep', 'force': '扭力→承载', 'vector': (1, 1)},
    'ia': {'direction': 'linear_to_radial', 'force': '拉力→发散', 'vector': (0.3, 1.3)},
    'ie': {'direction': 'linear_to_diagonal', 'force': '拉力→剪切', 'vector': (0.5, 1)},
    'ua': {'direction': 'pipe_to_radial', 'force': '压力→发散', 'vector': (0.5, 1.2)},
    'uo': {'direction': 'pipe_to_circular', 'force': '压力→扭力', 'vector': (1, 1)},
    'iao': {'direction': 'linear_to_circular', 'force': '拉力→扭力', 'vector': (1, 1.2)},
    'iu': {'direction': 'linear_to_pipe', 'force': '拉力→压力', 'vector': (0.8, 0.8)},
    'uai': {'direction': 'pipe_to_linear', 'force': '压力→拉力', 'vector': (0.5, 1.2)},
    'ue': {'direction': 'pipe_to_diagonal', 'force': '压力→剪切', 'vector': (0.8, 1)},
    'üe': {'direction': 'narrow_to_diagonal', 'force': '聚焦→剪切', 'vector': (0.7, 1)},
}

# --- 声调 ---
TONE_ENERGY = {
    1: 1.0,   # 一声: 高势能恒定
    2: 1.2,   # 二声: 势能上升
    3: 0.8,   # 三声: 势能降升
    4: 1.5,   # 四声: 势能下降
}


# ============================================================
# 解析器
# ============================================================

class GlyphParser:
    """字形几何语言解析器"""
    
    # 所有声母
    SHENGMU = 'bpmfdtnlgkhjqxzhchshrzcsyw'
    # 所有韵母
    YUNMU = ['a', 'o', 'e', 'i', 'u', 'ü', 'ai', 'ei', 'ao', 'ou', 'an', 'en',
             'ang', 'eng', 'ong', 'ia', 'ie', 'ua', 'uo', 'iao', 'iu', 'uai', 'ue', 'üe']
    
    def __init__(self):
        self.input_text = ""
        self.tokens = []
        self.ast = []
    
    def parse(self, text: str) -> List[Syllable]:
        """解析输入文本为音节序列"""
        self.input_text = text.strip()
        self.tokens = []
        self.ast = []
        
        # 简单解析：每两个字符为一个音节
        i = 0
        chars = list(self.input_text.replace(' ', ''))
        
        while i < len(chars):
            # 尝试匹配双字母声母
            sm = ''
            if i + 1 < len(chars):
                two = chars[i] + chars[i+1]
                if two in ('zh', 'ch', 'sh'):
                    sm = two
                    i += 2
            
            if not sm:
                if chars[i] in self.SHENGMU:
                    sm = chars[i]
                    i += 1
                else:
                    i += 1
                    continue
            
            # 匹配韵母（优先短韵母，避免吞掉下一个声母）
            ym = ''
            for ym_candidate in sorted(self.YUNMU, key=len):  # 短优先
                ym_chars = list(ym_candidate)
                if i + len(ym_chars) <= len(chars):
                    if chars[i:i+len(ym_chars)] == ym_chars:
                        # 检查：匹配后下一个字符是否还能作为声母开头
                        next_pos = i + len(ym_chars)
                        if next_pos < len(chars):
                            next_char = chars[next_pos]
                            # 如果下一个字符是声母开头，或者当前韵母是单字母，接受匹配
                            if next_char in self.SHENGMU or len(ym_chars) == 1:
                                ym = ym_candidate
                                i += len(ym_chars)
                                break
                        else:
                            ym = ym_candidate
                            i += len(ym_chars)
                            break
            
            if not ym:
                ym = 'a'  # 默认韵母
                i += 1
            
            # 解析声母和韵母
            if sm and ym:
                self.tokens.append({
                    'shengmu': sm,
                    'yunmu': ym,
                    'tone': 1,
                })
        
        return self.tokens
    
    def build_ast(self) -> List[Dict]:
        """构建抽象语法树"""
        self.ast = []
        for idx, token in enumerate(self.tokens):
            node = {
                'type': 'syllable',
                'index': idx,
                'shengmu': token['shengmu'],
                'yunmu': token['yunmu'],
                'tone': token['tone'],
                'force': self._calc_force(token),
                'vector': YUNMU_VECTORS.get(token['yunmu'], YUNMU_VECTORS['a']),
                'energy': TONE_ENERGY.get(token['tone'], 1.0),
            }
            self.ast.append(node)
        
        # 构建链接关系
        for i in range(len(self.ast) - 1):
            self.ast[i]['next'] = i + 1
            self.ast[i + 1]['prev'] = i
        
        return self.ast
    
    def _calc_force(self, token: Dict) -> float:
        """计算音节的力场强度"""
        sm = token['shengmu']
        tone = token['tone']
        
        # 获取声母力
        glyph = GLYPH_REGISTRY.get(sm)
        if not glyph:
            return 1.0
        
        base_force = glyph.force_strength
        
        # 声调能量系数
        energy = TONE_ENERGY.get(tone, 1.0)
        
        return base_force * energy


# ============================================================
# 执行引擎
# ============================================================

class GlyphExecutor:
    """字形几何语言执行引擎"""
    
    def __init__(self):
        self.parser = GlyphParser()
        self.state = FieldState()
        self.max_iterations = 100
        
    def execute(self, program: str) -> Dict:
        """执行程序"""
        # 1. 解析
        tokens = self.parser.parse(program)
        if not tokens:
            return {'ok': False, 'error': '无有效音节'}
        
        # 2. 构建AST
        ast = self.parser.build_ast()
        
        # 3. 初始化力场
        self.state = FieldState()
        for i, node in enumerate(ast):
            self.state.add_syllable(node, i)
        
        # 4. 力场演化
        for iteration in range(self.max_iterations):
            changes = self._evolve_step()
            self.state.iterations = iteration + 1
            
            if not changes:
                self.state.equilibrium = True
                break
        
        # 5. 返回结果
        return {
            'ok': True,
            'iterations': self.state.iterations,
            'equilibrium': self.state.equilibrium,
            'final_state': self._get_final_state(ast),
            'summary': self._summarize(ast),
        }
    
    def _evolve_step(self) -> bool:
        """一步力场演化"""
        changed = False
        ast = self.parser.ast
        
        # 先计算所有新力值
        new_forces = []
        for i, node in enumerate(ast):
            # 基础力
            base_force = self.parser._calc_force({
                'shengmu': node['shengmu'],
                'yunmu': node['yunmu'],
                'tone': node['tone'],
            })
            
            # 与邻居的相互作用（取平均，避免爆炸）
            neighbor_force = 0
            neighbor_count = 0
            
            if 'prev' in node:
                prev_node = ast[node['prev']]
                neighbor_force += prev_node.get('force', base_force)
                neighbor_count += 1
            
            if 'next' in node:
                next_node = ast[node['next']]
                neighbor_force += next_node.get('force', base_force)
                neighbor_count += 1
            
            if neighbor_count > 0:
                neighbor_force /= neighbor_count
            
            # 新力 = 基础力 + 邻居影响（衰减）
            old_force = node.get('force', base_force)
            new_force = base_force * 0.5 + neighbor_force * 0.3 + old_force * 0.2
            new_forces.append(new_force)
        
        # 更新
        for i, node in enumerate(ast):
            old_force = node.get('force', 0)
            node['force'] = new_forces[i]
            if abs(node['force'] - old_force) > 0.01:
                changed = True
        
        return changed
    
    def _calc_interaction(self, a: Dict, b: Dict) -> float:
        """计算两个音节之间的相互作用力"""
        force_a = a.get('force', 0)
        force_b = b.get('force', 0)
        # 加法统摄：相互作用 = 力之和
        return force_a + force_b
    
    def _get_final_state(self, ast: List[Dict]) -> List[Dict]:
        """获取最终状态"""
        result = []
        for node in ast:
            result.append({
                'syllable': f"{node['shengmu']}{node['yunmu']}",
                'force': round(node.get('force', 0), 2),
                'energy': node.get('energy', 1.0),
                'vector': node.get('vector', {}).get('direction', 'unknown'),
                'operation': GLYPH_REGISTRY.get(node['shengmu'], GlyphInfo('', [], '', '', '')).operation,
            })
        return result
    
    def _summarize(self, ast: List[Dict]) -> str:
        """总结执行结果"""
        total_force = sum(n.get('force', 0) for n in ast)
        total_energy = sum(n.get('energy', 1.0) for n in ast)
        return f"总力: {total_force:.2f}, 总能量: {total_energy:.2f}, 迭代: {self.state.iterations}"


# ============================================================
# 入口
# ============================================================

def run(program: str) -> Dict:
    """运行字形几何程序"""
    executor = GlyphExecutor()
    return executor.execute(program)


if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        program = sys.argv[1]
    else:
        program = "ba ma pa fa da ta na la"
    
    print(f"程序: {program}")
    print()
    
    result = run(program)
    
    if result['ok']:
        print(f"执行完成: {result['summary']}")
        print(f"平衡态: {'是' if result['equilibrium'] else '否'}")
        print()
        print("最终状态:")
        for state in result['final_state']:
            print(f"  {state['syllable']}: 力={state['force']}, 能量={state['energy']}, "
                  f"方向={state['vector']}")
            print(f"    操作: {state['operation']}")
    else:
        print(f"错误: {result['error']}")
