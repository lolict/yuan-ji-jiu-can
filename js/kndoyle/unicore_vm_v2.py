"""
UniCore v2.0 - 全新指令集虚拟机

特点：
1. 不进位计数执行
2. 节点因果时间系统
3. 集装箱内存管理
4. 插件指令扩展

版本: v2.0.0
日期: 2026-05-14
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from enum import Enum
import json


# 操作码定义
class OpCode(Enum):
    # 系统控制 (0x00-0x0F)
    NOP = 0x00
    HALT = 0x01
    LOAD = 0x02
    STORE = 0x03
    MOVE = 0x04
    COPY = 0x05
    
    # 不进位算术 (0x10-0x1F)
    INC = 0x10
    DEC = 0x11
    ADD = 0x12
    SUB = 0x13
    ZERO = 0x14
    MAX = 0x15
    FLIP = 0x16
    
    # 节点时间 (0x20-0x2F)
    CREATE_NODE = 0x20
    LINK_NODE = 0x21
    TRIGGER = 0x22
    WAIT_NODE = 0x23
    SYNC_TIME = 0x24
    GET_OFFSET = 0x25
    
    # 空间操作 (0x30-0x3F)
    SET_POS = 0x30
    GET_POS = 0x31
    MOVE_TO = 0x32
    DISTANCE = 0x33
    ROTATE = 0x34
    PROJECT = 0x35
    
    # 插件指令 (0x40-0xFF)
    LOAD_PLUGIN = 0x40
    UNLOAD_PLUGIN = 0x41
    CALL_PLUGIN = 0x42
    LIST_PLUGIN = 0x43


# 事件类型
class EventType(Enum):
    NORMAL = 0x01
    ACTION = 0x02
    STATE_CHANGE = 0x03
    CAUSAL_TRIGGER = 0x04


@dataclass
class BehaviorFlags:
    """8位时空/行为标记"""
    time_direction: bool = False  # Bit 7: 0=前向, 1=后向
    spatial_dimension: bool = False  # Bit 6: 0=3D, 1=多维
    operation_type: bool = False  # Bit 5: 0=读, 1=写
    sync_flag: bool = False  # Bit 4: 0=异步, 1=同步
    priority: bool = False  # Bit 3: 0=普通, 1=高
    parity: bool = False  # Bit 2: 0=偶校验, 1=奇校验
    ext1: bool = False  # Bit 1: 扩展位1
    ext2: bool = False  # Bit 0: 扩展位2
    
    def to_byte(self) -> int:
        """转换为字节"""
        result = 0
        if self.time_direction: result |= 0x80
        if self.spatial_dimension: result |= 0x40
        if self.operation_type: result |= 0x20
        if self.sync_flag: result |= 0x10
        if self.priority: result |= 0x08
        if self.parity: result |= 0x04
        if self.ext1: result |= 0x02
        if self.ext2: result |= 0x01
        return result
    
    @classmethod
    def from_byte(cls, byte: int) -> 'BehaviorFlags':
        """从字节创建"""
        flags = cls()
        flags.time_direction = bool(byte & 0x80)
        flags.spatial_dimension = bool(byte & 0x40)
        flags.operation_type = bool(byte & 0x20)
        flags.sync_flag = bool(byte & 0x10)
        flags.priority = bool(byte & 0x08)
        flags.parity = bool(byte & 0x04)
        flags.ext1 = bool(byte & 0x02)
        flags.ext2 = bool(byte & 0x01)
        return flags


class Container:
    """集装箱 - 最小存储单元"""
    
    MAX_VALUE = 255
    
    def __init__(self, container_id: int):
        self.id = container_id
        self.value: int = 0
        self.flags = BehaviorFlags()
        self.position = {'x': 0, 'y': 0, 'z': 0, 'w': 0}
        self.char_mapping: Optional[str] = None
    
    def non_carry_inc(self) -> int:
        """不进位加1"""
        self.value = (self.value + 1) % 256
        return self.value
    
    def non_carry_dec(self) -> int:
        """不进位减1"""
        self.value = (self.value - 1) % 256
        return self.value
    
    def non_carry_add(self, amount: int) -> int:
        """不进位加法"""
        self.value = (self.value + amount) % 256
        return self.value
    
    def non_carry_sub(self, amount: int) -> int:
        """不进位减法"""
        self.value = (self.value - amount) % 256
        return self.value
    
    def to_dict(self) -> Dict:
        """导出为字典"""
        return {
            'id': self.id,
            'value': self.value,
            'flags': self.flags.to_byte(),
            'position': self.position,
            'char_mapping': self.char_mapping
        }


class TimeNode:
    """节点 - 时间记录单元"""
    
    def __init__(self, node_id: int, event_type: EventType, 
                 cause_id: Optional[int] = None,
                 time_unit: str = 'UniTick'):
        self.id = node_id
        self.event_type = event_type
        self.cause_id = cause_id
        self.time_offset: float = 0.0
        self.time_unit = time_unit
        self.timestamp = 0.0
        self.triggered = False
    
    def to_dict(self) -> Dict:
        """导出为字典"""
        return {
            'id': self.id,
            'event_type': self.event_type.value,
            'cause_id': self.cause_id,
            'time_offset': self.time_offset,
            'time_unit': self.time_unit,
            'triggered': self.triggered
        }


class SpatialVector:
    """空间向量"""
    
    def __init__(self, x: int = 0, y: int = 0, z: int = 0, w: int = 0):
        self.x = max(-100, min(100, x))
        self.y = max(-100, min(100, y))
        self.z = max(-100, min(100, z))
        self.w = max(-100, min(100, w))
    
    def distance_to(self, other: 'SpatialVector') -> float:
        """计算到另一个向量的欧几里得距离"""
        dx = self.x - other.x
        dy = self.y - other.y
        dz = self.z - other.z
        dw = self.w - other.w
        return (dx*dx + dy*dy + dz*dz + dw*dw) ** 0.5
    
    def to_dict(self) -> Dict:
        return {'x': self.x, 'y': self.y, 'z': self.z, 'w': self.w}


class Plugin:
    """插件基类"""
    
    def __init__(self, plugin_id: int, name: str):
        self.id = plugin_id
        self.name = name
        self.loaded = False
    
    def execute(self, vm: 'UniCoreVM', args: List[int]) -> int:
        """执行插件逻辑"""
        raise NotImplementedError


class UniCoreVM:
    """UniCore v2.0 虚拟机核心"""
    
    def __init__(self):
        # 集装箱池 (256个)
        self.containers: List[Container] = [
            Container(i) for i in range(256)
        ]
        
        # 活动集装箱指针
        self.active_container_id: Optional[int] = None
        
        # 节点系统
        self.nodes: Dict[int, TimeNode] = {}
        self.node_counter = 0
        self.root_node: Optional[TimeNode] = None
        
        # 插件系统
        self.plugins: Dict[int, Plugin] = {}
        
        # 时间系统
        self.hardware_time_base = 0.0
        self.scale_factor = 1.0
        self.time_unit = 'UniTick'
        
        # 执行状态
        self.running = False
        self.pc = 0
        self.cycles = 0
        
        # 输出缓冲
        self.output: List[str] = []
        
        # 程序内存 (指令)
        self.program: List[int] = []
    
    def initialize(self):
        """初始化虚拟机"""
        self.running = True
        self.cycles = 0
        self.pc = 0
        self.hardware_time_base = 0.0
        
        # 创建根节点
        self.root_node = TimeNode(0, EventType.NORMAL, None, self.time_unit)
        self.nodes[0] = self.root_node
        self.node_counter = 1
        
        self.output.append(f"[VM] UniCore v2.0 initialized")
        self.output.append(f"[VM] Container pool: {len(self.containers)} containers")
        self.output.append(f"[VM] Root node created: ID=0")
    
    def load_program(self, program: List[int]):
        """加载程序"""
        self.program = program
        self.pc = 0
        self.output.append(f"[VM] Program loaded: {len(program)} bytes")
    
    def step(self) -> bool:
        """执行一条指令"""
        if not self.running or self.pc >= len(self.program):
            return False
        
        # 获取当前指令 (32位 = 4字节)
        if self.pc + 3 >= len(self.program):
            return False
        
        opcode_byte = self.program[self.pc]
        arg1 = self.program[self.pc + 1]
        arg2 = self.program[self.pc + 2]
        flags_byte = self.program[self.pc + 3]
        
        try:
            opcode = OpCode(opcode_byte)
        except ValueError:
            self.output.append(f"[ERROR] Unknown opcode: 0x{opcode_byte:02X}")
            return False
        
        # 解析参数
        container_id = arg1
        args = [arg1, arg2]
        flags = BehaviorFlags.from_byte(flags_byte)
        
        # 执行指令
        success = self._execute_opcode(opcode, container_id, args, flags)
        
        self.pc += 4
        self.cycles += 1
        
        return success and self.running
    
    def _execute_opcode(self, opcode: OpCode, container_id: int, 
                       args: List[int], flags: BehaviorFlags) -> bool:
        """执行操作码"""
        
        # 系统控制指令
        if opcode == OpCode.NOP:
            return True
        
        elif opcode == OpCode.HALT:
            self.running = False
            self.output.append(f"[VM] HALT at cycle {self.cycles}")
            return False
        
        elif opcode == OpCode.LOAD:
            if 0 <= container_id < 256:
                self.active_container_id = container_id
                self.output.append(f"[VM] LOAD container 0x{container_id:02X}, value={self.containers[container_id].value}")
            return True
        
        elif opcode == OpCode.STORE:
            if self.active_container_id is not None and 0 <= container_id < 256:
                self.containers[container_id].value = self.containers[self.active_container_id].value
                self.output.append(f"[VM] STORE to container 0x{container_id:02X}")
            return True
        
        elif opcode == OpCode.COPY:
            if 0 <= container_id < 256 and 0 <= args[0] < 256:
                self.containers[args[0]].value = self.containers[container_id].value
                self.output.append(f"[VM] COPY from 0x{container_id:02X} to 0x{args[0]:02X}")
            return True
        
        # 不进位算术指令
        elif opcode == OpCode.INC:
            if self.active_container_id is not None:
                new_val = self.containers[self.active_container_id].non_carry_inc()
                self.output.append(f"[VM] INC container 0x{self.active_container_id:02X} -> {new_val}")
            return True
        
        elif opcode == OpCode.DEC:
            if self.active_container_id is not None:
                new_val = self.containers[self.active_container_id].non_carry_dec()
                self.output.append(f"[VM] DEC container 0x{self.active_container_id:02X} -> {new_val}")
            return True
        
        elif opcode == OpCode.ADD:
            if self.active_container_id is not None:
                amount = container_id
                new_val = self.containers[self.active_container_id].non_carry_add(amount)
                self.output.append(f"[VM] ADD {amount} to container 0x{self.active_container_id:02X} -> {new_val}")
            return True
        
        elif opcode == OpCode.SUB:
            if self.active_container_id is not None:
                amount = container_id
                new_val = self.containers[self.active_container_id].non_carry_sub(amount)
                self.output.append(f"[VM] SUB {amount} from container 0x{self.active_container_id:02X} -> {new_val}")
            return True
        
        elif opcode == OpCode.ZERO:
            if self.active_container_id is not None:
                self.containers[self.active_container_id].value = 0
                self.output.append(f"[VM] ZERO container 0x{self.active_container_id:02X}")
            return True
        
        elif opcode == OpCode.MAX:
            if self.active_container_id is not None:
                self.containers[self.active_container_id].value = 255
                self.output.append(f"[VM] MAX container 0x{self.active_container_id:02X}")
            return True
        
        elif opcode == OpCode.FLIP:
            if self.active_container_id is not None:
                self.containers[self.active_container_id].value = 255 - self.containers[self.active_container_id].value
                self.output.append(f"[VM] FLIP container 0x{self.active_container_id:02X}")
            return True
        
        # 节点时间指令
        elif opcode == OpCode.CREATE_NODE:
            event_type_val = container_id
            cause_id = args[0] if args[0] != 0xFF else None
            
            event_type = EventType(event_type_val) if event_type_val in [e.value for e in EventType] else EventType.NORMAL
            
            node = TimeNode(self.node_counter, event_type, cause_id, self.time_unit)
            self.nodes[self.node_counter] = node
            self.node_counter += 1
            
            self.output.append(f"[VM] CREATE_NODE ID={node.id}, type={event_type.name}, cause={cause_id}")
            return True
        
        elif opcode == OpCode.TRIGGER:
            node_id = container_id
            if node_id in self.nodes:
                node = self.nodes[node_id]
                node.triggered = True
                
                # 因果传播
                if node.cause_id is not None and node.cause_id in self.nodes:
                    self.nodes[node.cause_id].triggered = True
                
                self.output.append(f"[VM] TRIGGER node {node_id}")
            return True
        
        elif opcode == OpCode.SYNC_TIME:
            import time
            self.hardware_time_base = time.time()
            self.output.append(f"[VM] SYNC_TIME, base={self.hardware_time_base}")
            return True
        
        # 空间操作指令
        elif opcode == OpCode.SET_POS:
            if self.active_container_id is not None:
                x, y, z = container_id, args[0], args[1]
                c = self.containers[self.active_container_id]
                c.position = {'x': x, 'y': y, 'z': z, 'w': 0}
                self.output.append(f"[VM] SET_POS container 0x{self.active_container_id:02X} -> ({x}, {y}, {z})")
            return True
        
        elif opcode == OpCode.GET_POS:
            if self.active_container_id is not None:
                c = self.containers[self.active_container_id]
                pos = c.position
                self.output.append(f"[VM] GET_POS container 0x{self.active_container_id:02X} -> ({pos['x']}, {pos['y']}, {pos['z']})")
            return True
        
        # 插件指令
        elif opcode == OpCode.LOAD_PLUGIN:
            plugin_id = container_id
            if plugin_id in self.plugins:
                self.plugins[plugin_id].loaded = True
                self.output.append(f"[VM] LOAD_PLUGIN 0x{plugin_id:02X} ({self.plugins[plugin_id].name})")
            return True
        
        elif opcode == OpCode.UNLOAD_PLUGIN:
            plugin_id = container_id
            if plugin_id in self.plugins:
                self.plugins[plugin_id].loaded = False
                self.output.append(f"[VM] UNLOAD_PLUGIN 0x{plugin_id:02X}")
            return True
        
        elif opcode == OpCode.CALL_PLUGIN:
            plugin_id = container_id
            if plugin_id in self.plugins and self.plugins[plugin_id].loaded:
                result = self.plugins[plugin_id].execute(self, args)
                self.output.append(f"[VM] CALL_PLUGIN 0x{plugin_id:02X} -> {result}")
            return True
        
        elif opcode == OpCode.LIST_PLUGIN:
            loaded = [f"0x{p.id:02X}({p.name})" for p in self.plugins.values() if p.loaded]
            self.output.append(f"[VM] Loaded plugins: {', '.join(loaded) if loaded else 'none'}")
            return True
        
        return True
    
    def register_plugin(self, plugin: Plugin):
        """注册插件"""
        self.plugins[plugin.id] = plugin
        self.output.append(f"[VM] Plugin registered: 0x{plugin.id:02X} ({plugin.name})")
    
    def run(self, max_cycles: int = 1000) -> List[str]:
        """运行程序"""
        self.initialize()
        self.output.append(f"[VM] Starting execution (max {max_cycles} cycles)")
        
        while self.running and self.cycles < max_cycles:
            if not self.step():
                break
        
        self.output.append(f"[VM] Execution finished after {self.cycles} cycles")
        return self.output
    
    def get_state(self) -> Dict:
        """获取虚拟机状态"""
        return {
            'running': self.running,
            'cycles': self.cycles,
            'pc': self.pc,
            'active_container': self.active_container_id,
            'container_count': len([c for c in self.containers if c.value != 0]),
            'node_count': len(self.nodes),
            'plugin_count': len([p for p in self.plugins.values() if p.loaded])
        }
    
    def export_state(self) -> str:
        """导出完整状态为JSON"""
        state = {
            'vm_version': 'v2.0.0',
            'vm_state': self.get_state(),
            'containers': [c.to_dict() for c in self.containers],
            'nodes': {k: v.to_dict() for k, v in self.nodes.items()},
            'plugins': {k: {'id': v.id, 'name': v.name, 'loaded': v.loaded} 
                        for k, v in self.plugins.items()},
            'output_log': self.output[-100:]  # 最后100条日志
        }
        return json.dumps(state, indent=2, ensure_ascii=False)
    
    def import_state(self, json_str: str):
        """从JSON导入状态"""
        state = json.loads(json_str)
        
        # 恢复集装箱状态
        for c_state in state.get('containers', []):
            cid = c_state['id']
            if 0 <= cid < 256:
                c = self.containers[cid]
                c.value = c_state['value']
                c.flags = BehaviorFlags.from_byte(c_state['flags'])
                c.position = c_state['position']
                c.char_mapping = c_state.get('char_mapping')
        
        # 恢复节点状态
        self.nodes.clear()
        for nid, n_state in state.get('nodes', {}).items():
            node = TimeNode(
                n_state['id'],
                EventType(n_state['event_type']),
                n_state['cause_id'],
                n_state['time_unit']
            )
            node.time_offset = n_state['time_offset']
            node.triggered = n_state['triggered']
            self.nodes[int(nid)] = node
        
        self.output.append("[VM] State imported successfully")


# 示例插件
class HelloWorldPlugin(Plugin):
    """Hello World 插件"""
    
    def __init__(self):
        super().__init__(0xA0, "HelloWorld")
    
    def execute(self, vm: UniCoreVM, args: List[int]) -> int:
        vm.output.append("Hello from HelloWorld plugin!")
        return 0


class CharMapPlugin(Plugin):
    """字符映射插件"""
    
    def __init__(self):
        super().__init__(0xA1, "CharMap")
    
    def execute(self, vm: UniCoreVM, args: List[int]) -> int:
        char = chr(vm.containers[vm.active_container_id].value)
        vm.output.append(f"Character: {char}")
        return vm.containers[vm.active_container_id].value


# 测试程序
def test_non_carry():
    """测试不进位计数"""
    vm = UniCoreVM()
    vm.initialize()
    
    # 测试程序
    program = [
        OpCode.LOAD.value, 0x10, 0x00, 0x00,  # LOAD container 0x10
        OpCode.INC.value, 0x00, 0x00, 0x00,   # INC
        OpCode.INC.value, 0x00, 0x00, 0x00,   # INC
        OpCode.INC.value, 0x00, 0x00, 0x00,   # INC
        OpCode.ADD.value, 0xFC, 0x00, 0x00,   # ADD 252 (255-3+0)
        OpCode.HALT.value, 0x00, 0x00, 0x00,  # HALT
    ]
    
    vm.load_program(program)
    output = vm.run()
    
    print("=== Non-Carry Test ===")
    for line in output:
        print(line)
    
    print(f"\nFinal state: {vm.get_state()}")
    print(f"Container 0x10 value: {vm.containers[0x10].value}")


def test_node_system():
    """测试节点系统"""
    vm = UniCoreVM()
    
    program = [
        OpCode.CREATE_NODE.value, EventType.NORMAL.value, 0xFF, 0x00,  # CREATE_NODE type=1, no cause
        OpCode.CREATE_NODE.value, EventType.ACTION.value, 0x01, 0x00,  # CREATE_NODE type=2, cause=1
        OpCode.TRIGGER.value, 0x02, 0x00, 0x00,                        # TRIGGER node 2
        OpCode.HALT.value, 0x00, 0x00, 0x00,
    ]
    
    vm.load_program(program)
    output = vm.run()
    
    print("=== Node System Test ===")
    for line in output:
        print(line)


def test_export_import():
    """测试导出导入"""
    vm = UniCoreVM()
    vm.initialize()
    
    # 加载插件
    vm.register_plugin(HelloWorldPlugin())
    vm.register_plugin(CharMapPlugin())
    
    # 设置一些集装箱值
    vm.containers[0].value = 72  # 'H'
    vm.containers[1].value = 101  # 'e'
    vm.containers[2].value = 108  # 'l'
    
    # 导出
    json_state = vm.export_state()
    
    print("=== Export/Import Test ===")
    print("State exported, length:", len(json_state))
    
    # 创建新VM并导入
    vm2 = UniCoreVM()
    vm2.import_state(json_state)
    
    print(f"Imported container 0 value: {vm2.containers[0].value}")
    print(f"Imported container 1 value: {vm2.containers[1].value}")
    print(f"Imported container 2 value: {vm2.containers[2].value}")


if __name__ == "__main__":
    print("UniCore v2.0 - 全新指令集虚拟机")
    print("=" * 50)
    
    test_non_carry()
    print()
    
    test_node_system()
    print()
    
    test_export_import()
