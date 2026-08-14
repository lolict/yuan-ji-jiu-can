//! UniCore v2.0 - Rust WASM 底层核心引擎
//! 
//! 底层优先级最高，包含：
//! - 不进位计数执行器
//! - 节点因果时间引擎
//! - 集装箱内存管理
//! - 自定义字符集改造系统
//! - 插件指令扩展

use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

/// ==================== 常量定义 ====================

const MAX_CONTAINERS: usize = 256;
const MAX_VALUE: u8 = 255;
const NODE_TIME_SCALE_DEFAULT: f64 = 1.0;

/// ==================== 行为标记 ====================

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct BehaviorFlags {
    time_direction: bool,     // Bit 7: 0=前向, 1=后向
    spatial_dimension: bool,    // Bit 6: 0=3D, 1=多维
    operation_type: bool,      // Bit 5: 0=读, 1=写
    sync_flag: bool,          // Bit 4: 0=异步, 1=同步
    priority: bool,           // Bit 3: 0=普通, 1=高
    parity: bool,             // Bit 2: 0=偶校验, 1=奇校验
    ext1: bool,              // Bit 1: 扩展位1
    ext2: bool,              // Bit 0: 扩展位2
}

#[wasm_bindgen]
impl BehaviorFlags {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BehaviorFlags {
        BehaviorFlags {
            time_direction: false,
            spatial_dimension: false,
            operation_type: false,
            sync_flag: false,
            priority: false,
            parity: false,
            ext1: false,
            ext2: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.time_direction { byte |= 0x80; }
        if self.spatial_dimension { byte |= 0x40; }
        if self.operation_type { byte |= 0x20; }
        if self.sync_flag { byte |= 0x10; }
        if self.priority { byte |= 0x08; }
        if self.parity { byte |= 0x04; }
        if self.ext1 { byte |= 0x02; }
        if self.ext2 { byte |= 0x01; }
        byte
    }

    pub fn from_byte(byte: u8) -> BehaviorFlags {
        BehaviorFlags {
            time_direction: (byte & 0x80) != 0,
            spatial_dimension: (byte & 0x40) != 0,
            operation_type: (byte & 0x20) != 0,
            sync_flag: (byte & 0x10) != 0,
            priority: (byte & 0x08) != 0,
            parity: (byte & 0x04) != 0,
            ext1: (byte & 0x02) != 0,
            ext2: (byte & 0x01) != 0,
        }
    }
}

impl Default for BehaviorFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// ==================== 集装箱 ====================

#[wasm_bindgen]
#[derive(Clone)]
pub struct Container {
    id: u8,
    value: u8,
    flags: BehaviorFlags,
    position_x: i32,
    position_y: i32,
    position_z: i32,
    position_w: i32,
}

#[wasm_bindgen]
impl Container {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u8) -> Container {
        Container {
            id,
            value: 0,
            flags: BehaviorFlags::new(),
            position_x: 0,
            position_y: 0,
            position_z: 0,
            position_w: 0,
        }
    }

    /// 不进位加1 (255 -> 0)
    pub fn non_carry_inc(&mut self) -> u8 {
        self.value = self.value.wrapping_add(1);
        self.value
    }

    /// 不进位减1 (0 -> 255)
    pub fn non_carry_dec(&mut self) -> u8 {
        self.value = self.value.wrapping_sub(1);
        self.value
    }

    /// 不进位加法
    pub fn non_carry_add(&mut self, amount: u8) -> u8 {
        self.value = self.value.wrapping_add(amount);
        self.value
    }

    /// 不进位减法
    pub fn non_carry_sub(&mut self, amount: u8) -> u8 {
        self.value = self.value.wrapping_sub(amount);
        self.value
    }

    pub fn get_id(&self) -> u8 { self.id }
    pub fn get_value(&self) -> u8 { self.value }
    pub fn set_value(&mut self, val: u8) { self.value = val; }
    pub fn get_flags_byte(&self) -> u8 { self.flags.to_byte() }
    pub fn set_flags_byte(&mut self, byte: u8) { self.flags = BehaviorFlags::from_byte(byte); }
    pub fn get_position_x(&self) -> i32 { self.position_x }
    pub fn get_position_y(&self) -> i32 { self.position_y }
    pub fn get_position_z(&self) -> i32 { self.position_z }
    pub fn set_position(&mut self, x: i32, y: i32, z: i32) {
        self.position_x = x.clamp(-100, 100);
        self.position_y = y.clamp(-100, 100);
        self.position_z = z.clamp(-100, 100);
    }
}

/// ==================== 节点 ====================

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventType {
    Normal = 1,
    Action = 2,
    StateChange = 3,
    CausalTrigger = 4,
}

#[wasm_bindgen]
impl EventType {
    pub fn from_u8(val: u8) -> EventType {
        match val {
            1 => EventType::Normal,
            2 => EventType::Action,
            3 => EventType::StateChange,
            4 => EventType::CausalTrigger,
            _ => EventType::Normal,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TimeNode {
    id: u32,
    event_type: EventType,
    cause_id: Option<u32>,
    time_offset: f64,
    time_unit: String,
    triggered: bool,
}

#[wasm_bindgen]
impl TimeNode {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, event_type: u8, cause_id: Option<u32>) -> TimeNode {
        TimeNode {
            id,
            event_type: EventType::from_u8(event_type),
            cause_id,
            time_offset: 0.0,
            time_unit: "UniTick".to_string(),
            triggered: false,
        }
    }

    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_event_type(&self) -> u8 { self.event_type as u8 }
    pub fn get_cause_id(&self) -> Option<u32> { self.cause_id }
    pub fn get_time_offset(&self) -> f64 { self.time_offset }
    pub fn set_time_offset(&mut self, offset: f64) { self.time_offset = offset; }
    pub fn is_triggered(&self) -> bool { self.triggered }
    pub fn trigger(&mut self) { self.triggered = true; }
}

/// ==================== 操作码 ====================

#[wasm_bindgen]
pub enum OpCode {
    Nop = 0x00,
    Halt = 0x01,
    Load = 0x02,
    Store = 0x03,
    Move = 0x04,
    Copy = 0x05,
    Inc = 0x10,
    Dec = 0x11,
    Add = 0x12,
    Sub = 0x13,
    Zero = 0x14,
    Max = 0x15,
    Flip = 0x16,
    CreateNode = 0x20,
    LinkNode = 0x21,
    Trigger = 0x22,
    WaitNode = 0x23,
    SyncTime = 0x24,
    GetOffset = 0x25,
    SetPos = 0x30,
    GetPos = 0x31,
    MoveTo = 0x32,
    Distance = 0x33,
    Rotate = 0x34,
    Project = 0x35,
    LoadPlugin = 0x40,
    UnloadPlugin = 0x41,
    CallPlugin = 0x42,
    ListPlugin = 0x43,
}

impl OpCode {
    pub fn from_u8(val: u8) -> Option<OpCode> {
        match val {
            0x00 => Some(OpCode::Nop),
            0x01 => Some(OpCode::Halt),
            0x02 => Some(OpCode::Load),
            0x03 => Some(OpCode::Store),
            0x04 => Some(OpCode::Move),
            0x05 => Some(OpCode::Copy),
            0x10 => Some(OpCode::Inc),
            0x11 => Some(OpCode::Dec),
            0x12 => Some(OpCode::Add),
            0x13 => Some(OpCode::Sub),
            0x14 => Some(OpCode::Zero),
            0x15 => Some(OpCode::Max),
            0x16 => Some(OpCode::Flip),
            0x20 => Some(OpCode::CreateNode),
            0x21 => Some(OpCode::LinkNode),
            0x22 => Some(OpCode::Trigger),
            0x23 => Some(OpCode::WaitNode),
            0x24 => Some(OpCode::SyncTime),
            0x25 => Some(OpCode::GetOffset),
            0x30 => Some(OpCode::SetPos),
            0x31 => Some(OpCode::GetPos),
            0x32 => Some(OpCode::MoveTo),
            0x33 => Some(OpCode::Distance),
            0x34 => Some(OpCode::Rotate),
            0x35 => Some(OpCode::Project),
            0x40 => Some(OpCode::LoadPlugin),
            0x41 => Some(OpCode::UnloadPlugin),
            0x42 => Some(OpCode::CallPlugin),
            0x43 => Some(OpCode::ListPlugin),
            _ => None,
        }
    }
}

/// ==================== 插件接口 ====================

#[wasm_bindgen]
pub trait Plugin: {
    fn get_id(&self) -> u8;
    fn get_name(&self) -> String;
    fn execute(&mut self, vm: &mut UniCoreVM, args: &[u8]) -> u8;
}

/// ==================== UniCore VM 核心 ====================

#[wasm_bindgen]
pub struct UniCoreVM {
    containers: Vec<Container>,
    active_container_id: Option<u8>,
    nodes: HashMap<u32, TimeNode>,
    node_counter: u32,
    root_node_id: u32,
    time_scale: f64,
    time_unit: String,
    hardware_time_base: f64,
    running: bool,
    cycles: u64,
    pc: usize,
    program: Vec<u8>,
    output_log: Vec<String>,
}

#[wasm_bindgen]
impl UniCoreVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UniCoreVM {
        let containers: Vec<Container> = (0..MAX_CONTAINERS as u8)
            .map(Container::new)
            .collect();
        
        UniCoreVM {
            containers,
            active_container_id: None,
            nodes: HashMap::new(),
            node_counter: 1,
            root_node_id: 0,
            time_scale: NODE_TIME_SCALE_DEFAULT,
            time_unit: "UniTick".to_string(),
            hardware_time_base: 0.0,
            running: false,
            cycles: 0,
            pc: 0,
            program: Vec::new(),
            output_log: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.running = true;
        self.cycles = 0;
        self.pc = 0;
        self.hardware_time_base = js_sys::Date::now() / 1000.0;
        
        // 创建根节点
        let root = TimeNode::new(0, 1, None);
        self.nodes.insert(0, root);
        self.root_node_id = 0;
        self.node_counter = 1;
        
        self.log("UniCore v2.0 WASM initialized");
        self.log(&format!("Container pool: {} containers", MAX_CONTAINERS));
        self.log("Root node created: ID=0");
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.program = program.to_vec();
        self.pc = 0;
        self.log(&format!("Program loaded: {} bytes", self.program.len()));
    }

    pub fn step(&mut self) -> bool {
        if !self.running || self.pc >= self.program.len() {
            return false;
        }

        // 32位指令 = 4字节
        if self.pc + 3 >= self.program.len() {
            return false;
        }

        let opcode_byte = self.program[self.pc];
        let arg1 = self.program[self.pc + 1];
        let arg2 = self.program[self.pc + 2];
        let flags_byte = self.program[self.pc + 3];

        if let Some(opcode) = OpCode::from_u8(opcode_byte) {
            self.execute_opcode(opcode, arg1, arg2, flags_byte);
        } else {
            self.log(&format!("Unknown opcode: 0x{:02X}", opcode_byte));
        }

        self.pc += 4;
        self.cycles += 1;
        
        self.running
    }

    fn execute_opcode(&mut self, opcode: OpCode, arg1: u8, arg2: u8, _flags: u8) {
        match opcode {
            OpCode::Nop => {}
            
            OpCode::Halt => {
                self.running = false;
                self.log(&format!("HALT at cycle {}", self.cycles));
            }
            
            OpCode::Load => {
                if (arg1 as usize) < MAX_CONTAINERS {
                    self.active_container_id = Some(arg1);
                    self.log(&format!("LOAD container 0x{:02X}, value={}", 
                        arg1, self.containers[arg1 as usize].get_value()));
                }
            }
            
            OpCode::Store => {
                if let Some(active) = self.active_container_id {
                    if (arg1 as usize) < MAX_CONTAINERS {
                        self.containers[arg1 as usize].set_value(self.containers[active as usize].get_value());
                        self.log(&format!("STORE to container 0x{:02X}", arg1));
                    }
                }
            }
            
            OpCode::Copy => {
                if (arg1 as usize) < MAX_CONTAINERS && (arg2 as usize) < MAX_CONTAINERS {
                    let val = self.containers[arg1 as usize].get_value();
                    self.containers[arg2 as usize].set_value(val);
                    self.log(&format!("COPY from 0x{:02X} to 0x{:02X}", arg1, arg2));
                }
            }
            
            OpCode::Inc => {
                if let Some(id) = self.active_container_id {
                    let new_val = self.containers[id as usize].non_carry_inc();
                    self.log(&format!("INC container 0x{:02X} -> {}", id, new_val));
                }
            }
            
            OpCode::Dec => {
                if let Some(id) = self.active_container_id {
                    let new_val = self.containers[id as usize].non_carry_dec();
                    self.log(&format!("DEC container 0x{:02X} -> {}", id, new_val));
                }
            }
            
            OpCode::Add => {
                if let Some(id) = self.active_container_id {
                    let new_val = self.containers[id as usize].non_carry_add(arg1);
                    self.log(&format!("ADD {} to container 0x{:02X} -> {}", arg1, id, new_val));
                }
            }
            
            OpCode::Sub => {
                if let Some(id) = self.active_container_id {
                    let new_val = self.containers[id as usize].non_carry_sub(arg1);
                    self.log(&format!("SUB {} from container 0x{:02X} -> {}", arg1, id, new_val));
                }
            }
            
            OpCode::Zero => {
                if let Some(id) = self.active_container_id {
                    self.containers[id as usize].set_value(0);
                    self.log(&format!("ZERO container 0x{:02X}", id));
                }
            }
            
            OpCode::Max => {
                if let Some(id) = self.active_container_id {
                    self.containers[id as usize].set_value(255);
                    self.log(&format!("MAX container 0x{:02X}", id));
                }
            }
            
            OpCode::Flip => {
                if let Some(id) = self.active_container_id {
                    let val = self.containers[id as usize].get_value();
                    self.containers[id as usize].set_value(255 - val);
                    self.log(&format!("FLIP container 0x{:02X} -> {}", id, 255 - val));
                }
            }
            
            OpCode::CreateNode => {
                let cause_id = if arg2 == 0xFF { None } else { Some(arg2 as u32) };
                let node = TimeNode::new(self.node_counter, arg1, cause_id);
                self.nodes.insert(self.node_counter, node);
                self.node_counter += 1;
                self.log(&format!("CREATE_NODE ID={}, type={}, cause={:?}", 
                    self.node_counter - 1, arg1, cause_id));
            }
            
            OpCode::Trigger => {
                let node_id = arg1 as u32;
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.trigger();
                    // 因果传播
                    if let Some(cause_id) = node.cause_id {
                        if let Some(cause) = self.nodes.get_mut(&cause_id) {
                            cause.trigger();
                        }
                    }
                    self.log(&format!("TRIGGER node {}", node_id));
                }
            }
            
            OpCode::SyncTime => {
                self.hardware_time_base = js_sys::Date::now() / 1000.0;
                self.log(&format!("SYNC_TIME, base={}", self.hardware_time_base));
            }
            
            OpCode::SetPos => {
                if let Some(id) = self.active_container_id {
                    self.containers[id as usize].set_position(arg1 as i32, arg2 as i32, 0);
                    self.log(&format!("SET_POS container 0x{:02X} -> ({}, {}, 0)", id, arg1, arg2));
                }
            }
            
            OpCode::GetPos => {
                if let Some(id) = self.active_container_id {
                    let c = &self.containers[id as usize];
                    self.log(&format!("GET_POS container 0x{:02X} -> ({}, {}, {})", 
                        id, c.get_position_x(), c.get_position_y(), c.get_position_z()));
                }
            }
            
            OpCode::LoadPlugin => {
                self.log(&format!("LOAD_PLUGIN 0x{:02X}", arg1));
            }
            
            OpCode::CallPlugin => {
                self.log(&format!("CALL_PLUGIN 0x{:02X}", arg1));
            }
            
            _ => {
                self.log(&format!("Opcode 0x{:02X} not implemented", opcode as u8));
            }
        }
    }

    pub fn run(&mut self, max_cycles: u64) {
        self.initialize();
        self.log(&format!("Starting execution (max {} cycles)", max_cycles));
        
        while self.running && self.cycles < max_cycles {
            if !self.step() {
                break;
            }
        }
        
        self.log(&format!("Execution finished after {} cycles", self.cycles));
    }

    fn log(&mut self, msg: &str) {
        self.output_log.push(msg.to_string());
    }

    // ==================== 状态查询 ====================
    
    pub fn is_running(&self) -> bool { self.running }
    pub fn get_cycles(&self) -> u64 { self.cycles }
    pub fn get_active_container(&self) -> Option<u8> { self.active_container_id }
    pub fn get_container_value(&self, id: u8) -> u8 { 
        if (id as usize) < MAX_CONTAINERS {
            self.containers[id as usize].get_value()
        } else {
            0
        }
    }
    pub fn get_node_count(&self) -> u32 { self.nodes.len() as u32 }
    pub fn get_output_log(&self) -> String { 
        self.output_log.join("\n") 
    }
    
    pub fn get_container_state(&self, id: u8) -> JsValue {
        if (id as usize) < MAX_CONTAINERS {
            let c = &self.containers[id as usize];
            js_sys::Object::from([
                ("id", JsValue::from(c.get_id())),
                ("value", JsValue::from(c.get_value())),
                ("flags", JsValue::from(c.get_flags_byte())),
                ("x", JsValue::from(c.get_position_x())),
                ("y", JsValue::from(c.get_position_y())),
                ("z", JsValue::from(c.get_position_z())),
            ].iter().map(|(k, v)| {
                let obj = js_sys::Object::new();
                Reflect::set(&obj, &JsValue::from(k), &v).unwrap();
                obj
            }).collect::<Vec<_>>()[0].clone())
        } else {
            JsValue::UNDEFINED
        }
    }
}

/// ==================== 自定义字符集改造系统 ====================

#[wasm_bindgen]
pub struct CustomCharset {
    // 字符映射表: 原始ASCII值 -> 自定义值
    char_to_custom: HashMap<u8, u8>,
    // 逆映射: 自定义值 -> 原始ASCII值
    custom_to_char: HashMap<u8, u8>,
    // 绑定到进制
    radix_binding: u8,
    // 绑定到集装箱范围
    container_start: u8,
    container_end: u8,
}

#[wasm_bindgen]
impl CustomCharset {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CustomCharset {
        CustomCharset {
            char_to_custom: HashMap::new(),
            custom_to_char: HashMap::new(),
            radix_binding: 64, // 默认绑定到64进制
            container_start: 0,
            container_end: 255,
        }
    }

    /// 绑定字符到自定义值
    pub fn bind_char(&mut self, ascii_char: u8, custom_value: u8) {
        self.char_to_custom.insert(ascii_char, custom_value);
        self.custom_to_char.insert(custom_value, ascii_char);
    }

    /// 绑定到进制系统
    pub fn bind_radix(&mut self, radix: u8) {
        self.radix_binding = radix;
    }

    /// 绑定到集装箱范围
    pub fn bind_container_range(&mut self, start: u8, end: u8) {
        self.container_start = start;
        self.container_end = end;
    }

    /// 转换为自定义编码
    pub fn encode(&self, text: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for c in text.bytes() {
            if let Some(&custom_val) = self.char_to_custom.get(&c) {
                result.push(custom_val);
            } else {
                result.push(c); // 保持原样
            }
        }
        result
    }

    /// 从自定义编码解码
    pub fn decode(&self, data: &[u8]) -> String {
        let mut result = Vec::new();
        for &custom_val in data {
            if let Some(&ascii_char) = self.custom_to_char.get(&custom_val) {
                result.push(ascii_char);
            } else {
                result.push(custom_val);
            }
        }
        String::from_utf8_lossy(&result).to_string()
    }

    /// 获取编码值
    pub fn get_custom_value(&self, ascii_char: u8) -> Option<u8> {
        self.char_to_custom.get(&ascii_char).copied()
    }

    /// 获取原始字符
    pub fn get_ascii_char(&self, custom_value: u8) -> Option<u8> {
        self.custom_to_char.get(&custom_value).copied()
    }
}

/// ==================== 全局状态 ====================

#[wasm_bindgen]
pub struct UniCoreGlobal {
    vm: UniCoreVM,
    charset: CustomCharset,
    initialized: bool,
}

#[wasm_bindgen]
impl UniCoreGlobal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UniCoreGlobal {
        UniCoreGlobal {
            vm: UniCoreVM::new(),
            charset: CustomCharset::new(),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if !self.initialized {
            self.vm.initialize();
            self.initialized = true;
        }
    }

    pub fn get_vm(&mut self) -> UniCoreVM {
        std::mem::replace(&mut self.vm, UniCoreVM::new())
    }

    pub fn get_charset(&mut self) -> CustomCharset {
        std::mem::replace(&mut self.charset, CustomCharset::new())
    }
}

// WASM入口点
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
}
