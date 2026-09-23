// ======================================================================
// celestial_perception_v13.rs — 天觉系统 v13（老十三）· WASM编译目标 + 蚕蛹寄生壳运行时
//
// 本文件在v12活码执行基础上, 赋予袋子跨平台寄生能力:
//
//   1. WASM编译目标 (WasmCompiler)          — InstructionBag → WASM字节码(wasm32/wasm64双向公约)
//   2. 蚕蛹寄生壳运行时 (ParasiteShell)     — 脱壳→选壳→穿衣→挂载内核 → 外部系统反向模仿
//   3. 寄生管理器 (ParasiteManager)         — 多壳并行 · 心跳恢复 · 配置即代码 · 跨宿主迁移
//
// 公理: 自身外部系统模仿自身, 而非自身适应外部规则;
//       同一份源码编译wasm32/wasm64, 架构无关.
// ======================================================================

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::celestial_perception_v12::{InstructionBag, OpCode};

pub const V13_VERSION: u32 = 13;

// ======================================================================
// 第一部分: WASM 编译目标 — 指令袋 → WASM 字节码
// ======================================================================

/// WASM 目标架构 — 双向公约
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmTarget {
    Wasm32,  // 32位 (嵌入式 / 浏览器)
    Wasm64,  // 64位 (服务端 / 大内存)
}

impl WasmTarget {
    /// 指针宽度(字节)
    pub fn pointer_size(&self) -> u8 {
        match self {
            WasmTarget::Wasm32 => 4,
            WasmTarget::Wasm64 => 8,
        }
    }

    /// 类型编码 — 架构无关的固定宽度
    pub fn value_type(&self) -> u8 {
        0x7F  // i64 (统一使用i64避免宽度差异)
    }
}

impl fmt::Display for WasmTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmTarget::Wasm32 => write!(f, "wasm32"),
            WasmTarget::Wasm64 => write!(f, "wasm64"),
        }
    }
}

/// WASM 模块 — 二进制编码的最小单元
#[derive(Clone, Debug)]
pub struct WasmModule {
    pub target: WasmTarget,
    pub bytes: Vec<u8>,
    pub source_bag_id: u64,
    pub source_ib_len: usize,
    pub symbol_table: HashMap<String, u32>,  // 名称 → 字节偏移
    pub entry_point: Option<u32>,
    pub compile_time_ms: u64,
    pub memory_pages: u32,     // 分配的内存页数
}

impl WasmModule {
    pub fn new(target: WasmTarget, source_bag_id: u64) -> Self {
        Self {
            target,
            bytes: Vec::new(),
            source_bag_id,
            source_ib_len: 0,
            symbol_table: HashMap::new(),
            entry_point: None,
            compile_time_ms: 0,
            memory_pages: 1,  // 默认1页(64KB)
        }
    }

    /// WASM魔术数字 + 版本 (二进制文件头)
    pub fn header(&self) -> Vec<u8> {
        vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]
    }

    /// 模块总字节数
    pub fn len(&self) -> usize { self.bytes.len() }
    pub fn is_empty(&self) -> bool { self.bytes.is_empty() }

    /// 是否包含有效的WASM文件头
    pub fn has_valid_header(&self) -> bool {
        self.bytes.len() >= 8 && self.bytes[0..4] == [0x00, 0x61, 0x73, 0x6D]
    }

    /// 符号表中查找
    pub fn resolve_symbol(&self, name: &str) -> Option<u32> {
        self.symbol_table.get(name).copied()
    }

    /// 架构摘要
    pub fn arch_summary(&self) -> String {
        format!(
            "WASM {} · {} bytes · {} syms · {} pages · bag#{}",
            self.target, self.bytes.len(), self.symbol_table.len(),
            self.memory_pages, self.source_bag_id,
        )
    }
}

/// WASM 计算栈指令 — 操作码 → WASM 字节码映射
const WASM_I64_CONST: u8 = 0x42;
const WASM_I64_ADD: u8 = 0x7C;
const WASM_I64_AND: u8 = 0x71;
const WASM_I64_OR: u8 = 0x72;
const WASM_I64_SHR: u8 = 0x86;
const WASM_I64_SHL: u8 = 0x85;
const WASM_I64_MUL: u8 = 0x7E;
const WASM_I64_DIV: u8 = 0x80;
const WASM_LOCAL_GET: u8 = 0x20;
const WASM_LOCAL_SET: u8 = 0x21;
const WASM_I64_LOAD: u8 = 0x29;
const WASM_I64_STORE: u8 = 0x37;
const WASM_BR_IF: u8 = 0x0D;
const WASM_BR: u8 = 0x0C;
const WASM_END: u8 = 0x0B;
const WASM_DROP: u8 = 0x1A;
const WASM_RETURN: u8 = 0x0F;
const WASM_CALL: u8 = 0x10;

/// WASM编译器 — InstructionBag → WASM字节码
#[derive(Clone, Debug)]
pub struct WasmCompiler {
    pub target: WasmTarget,
    pub compile_count: u64,
    pub dual_compile: bool,  // 是否双向编译(wasm32+wasm64)
}

impl WasmCompiler {
    pub fn new(target: WasmTarget) -> Self {
        Self {
            target,
            compile_count: 0,
            dual_compile: false,
        }
    }

    /// 创建双向编译器 (同时产出wasm32+wasm64)
    pub fn new_dual() -> Self {
        Self {
            target: WasmTarget::Wasm32,
            compile_count: 0,
            dual_compile: true,
        }
    }

    /// 编码变长无符号整数 (LEB128)
    fn encode_leb128(buf: &mut Vec<u8>, value: u64) -> usize {
        let start = buf.len();
        let mut v = value;
        loop {
            let mut byte = (v & 0x7F) as u8;
            v >>= 7;
            if v != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if v == 0 { break; }
        }
        buf.len() - start
    }

    /// 编码变长有符号整数 (LEB128 signed)
    fn encode_sleb128(buf: &mut Vec<u8>, value: i64) -> usize {
        let start = buf.len();
        let mut v = value;
        loop {
            let mut byte = (v & 0x7F) as u8;
            v >>= 7;
            let done = (v == 0 && (byte & 0x40) == 0) || (v == -1 && (byte & 0x40) != 0);
            if !done {
                byte |= 0x80;
            }
            buf.push(byte);
            if done { break; }
        }
        buf.len() - start
    }

    /// 编码字符串 (长度前缀 + UTF-8)
    fn encode_name(buf: &mut Vec<u8>, s: &str) {
        Self::encode_leb128(buf, s.len() as u64);
        buf.extend_from_slice(s.as_bytes());
    }

    /// 编译单个OpCode为WASM指令
    fn compile_opcode(buf: &mut Vec<u8>, op: &OpCode, operands: &[u64]) {
        match op {
            OpCode::LoadBag | OpCode::Spawn => {
                if let Some(&id) = operands.first() {
                    buf.push(WASM_I64_CONST);
                    Self::encode_sleb128(buf, id as i64);
                }
                buf.push(WASM_LOCAL_SET);
                buf.push(0); // local 0: bag_id
            }
            OpCode::AddValue => {
                if let Some(&val) = operands.first() {
                    buf.push(WASM_LOCAL_GET);
                    buf.push(1); // local 1: accumulator
                    buf.push(WASM_I64_CONST);
                    Self::encode_sleb128(buf, val as i64);
                    buf.push(WASM_I64_ADD);
                    buf.push(WASM_LOCAL_SET);
                    buf.push(1);
                }
            }
            OpCode::AddPolarity => {
                if operands.len() >= 2 {
                    Self::encode_sleb128_into(buf, operands[0] as i64);
                    Self::encode_sleb128_into(buf, operands[1] as i64);
                    buf.push(WASM_I64_ADD);
                    buf.push(WASM_LOCAL_SET);
                    buf.push(2);
                }
            }
            OpCode::CombineSovereignty => {
                if operands.len() >= 2 {
                    Self::encode_sleb128_into(buf, operands[0] as i64);
                    Self::encode_sleb128_into(buf, operands[1] as i64);
                    buf.push(WASM_I64_OR);
                    buf.push(WASM_LOCAL_SET);
                    buf.push(3);
                }
            }
            OpCode::JudgeAnd => {
                buf.push(WASM_LOCAL_GET);
                buf.push(0);
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 0);
                buf.push(WASM_I64_AND);  // 简化: 用AND模拟
            }
            OpCode::JudgeOr => {
                buf.push(WASM_LOCAL_GET);
                buf.push(0);
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 0);
                buf.push(WASM_I64_OR);
            }
            OpCode::Eliminate | OpCode::FlowReturn => {
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 0); // 返回0 = 消除
                buf.push(WASM_RETURN);
            }
            OpCode::PassThrough | OpCode::FlowProceed => {
                buf.push(WASM_NOP); // 哨兵
            }
            OpCode::FlowSink => {
                buf.push(WASM_LOCAL_GET);
                buf.push(1);
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 2);
                buf.push(WASM_I64_SHR);  // value >>= 1 (折半)
                buf.push(WASM_LOCAL_SET);
                buf.push(1);
            }
            OpCode::FlowFloat => {
                buf.push(WASM_LOCAL_GET);
                buf.push(1);
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 2);
                buf.push(WASM_I64_SHL);  // value <<= 1 (翻倍)
                buf.push(WASM_LOCAL_SET);
                buf.push(1);
            }
            OpCode::StoreBag | OpCode::Suspend | OpCode::Resume => {
                buf.push(WASM_LOCAL_GET);
                buf.push(0);
                buf.push(WASM_DROP);
            }
            OpCode::Destroy => {
                buf.push(WASM_RETURN);
            }
            OpCode::CreateChild | OpCode::MergeBags => {
                if operands.len() >= 2 {
                    Self::encode_sleb128_into(buf, operands[0] as i64);
                    Self::encode_sleb128_into(buf, operands[1] as i64);
                    buf.push(WASM_I64_ADD);
                    buf.push(WASM_LOCAL_SET);
                    buf.push(4);
                }
            }
            OpCode::SplitBag => {
                buf.push(WASM_LOCAL_GET);
                buf.push(1);
                buf.push(WASM_I64_CONST);
                Self::encode_sleb128(buf, 2);
                buf.push(WASM_I64_DIV);  // value / 2
                buf.push(WASM_LOCAL_SET);
                buf.push(1);
            }
            OpCode::MoveAttr | OpCode::MirrorAttr => {
                if let Some(&idx) = operands.first() {
                    buf.push(WASM_LOCAL_GET);
                    buf.push(0);
                    buf.push(WASM_I64_CONST);
                    Self::encode_sleb128(buf, idx as i64);
                    buf.push(WASM_I64_ADD);
                    buf.push(WASM_LOCAL_SET);
                    buf.push(5);
                }
            }
        }
    }

    fn encode_sleb128_into(buf: &mut Vec<u8>, val: i64) {
        buf.push(WASM_I64_CONST);
        Self::encode_sleb128(buf, val);
    }

    /// 编译指令袋为WASM模块
    pub fn compile(&mut self, ib: &InstructionBag) -> WasmModule {
        let mut module = WasmModule::new(self.target, ib.source_bag_id);
        module.source_ib_len = ib.len();
        let start = current_time_ms();

        // 1. 文件头
        module.bytes = module.header();

        // 2. Type Section (section id=1)
        self.emit_type_section(&mut module.bytes);

        // 3. Function Section (section id=3)
        self.emit_function_section(&mut module.bytes);

        // 4. Memory Section (section id=5)
        self.emit_memory_section(&mut module.bytes, module.memory_pages);

        // 5. Export Section (section id=7)
        self.emit_export_section(&mut module.bytes, ib.source_bag_id);

        // 6. Code Section (section id=10)
        self.emit_code_section(&mut module.bytes, ib);

        // 7. 符号表
        module.symbol_table.insert(format!("bag_{}", ib.source_bag_id), 0);
        module.symbol_table.insert("main".to_string(), 0);
        module.entry_point = Some(0);

        module.compile_time_ms = current_time_ms() - start;
        self.compile_count += 1;
        module
    }

    /// 双向编译 — 同时产出 wasm32 和 wasm64
    pub fn compile_dual(&mut self, ib: &InstructionBag) -> (WasmModule, WasmModule) {
        let mut c32 = Self::new(WasmTarget::Wasm32);
        let mut c64 = Self::new(WasmTarget::Wasm64);
        let m32 = c32.compile(ib);
        let m64 = c64.compile(ib);
        self.compile_count += 2;
        (m32, m64)
    }

    fn emit_type_section(&self, buf: &mut Vec<u8>) {
        buf.push(0x01); // section id: Type
        let mut section = Vec::new();
        // 类型0: (i64, i64, i64) → i64 (接受3参数，返回1值)
        Self::encode_leb128(&mut section, 1);  // 1个类型
        section.push(0x60); // functype
        section.push(0x03); // 3个参数
        section.push(self.target.value_type()); // i64
        section.push(self.target.value_type());
        section.push(self.target.value_type());
        section.push(0x01); // 1个返回值
        section.push(self.target.value_type()); // i64
        Self::encode_leb128(buf, section.len() as u64);
        buf.extend(section);
    }

    fn emit_function_section(&self, buf: &mut Vec<u8>) {
        buf.push(0x03); // section id: Function
        let mut section = Vec::new();
        Self::encode_leb128(&mut section, 1); // 1个函数
        Self::encode_leb128(&mut section, 0); // 类型索引0
        Self::encode_leb128(buf, section.len() as u64);
        buf.extend(section);
    }

    fn emit_memory_section(&self, buf: &mut Vec<u8>, pages: u32) {
        buf.push(0x05); // section id: Memory
        let mut section = Vec::new();
        Self::encode_leb128(&mut section, 1); // 1个内存
        section.push(0x00); // 无最大值
        Self::encode_leb128(&mut section, pages as u64);
        Self::encode_leb128(buf, section.len() as u64);
        buf.extend(section);
    }

    fn emit_export_section(&self, buf: &mut Vec<u8>, bag_id: u64) {
        buf.push(0x07); // section id: Export
        let mut section = Vec::new();
        Self::encode_leb128(&mut section, 1); // 1个导出
        let export_name = format!("bag_{}", bag_id);
        Self::encode_name(&mut section, &export_name);
        section.push(0x00); // export kind: function
        Self::encode_leb128(&mut section, 0); // function index 0
        Self::encode_leb128(buf, section.len() as u64);
        buf.extend(section);
    }

    fn emit_code_section(&self, buf: &mut Vec<u8>, ib: &InstructionBag) {
        buf.push(0x0A); // section id: Code
        let mut section = Vec::new();
        Self::encode_leb128(&mut section, 1); // 1个函数体

        let mut body = Vec::new();
        // 局部变量声明
        Self::encode_leb128(&mut body, 6); // 6组局部变量
        Self::encode_leb128(&mut body, 3); body.push(self.target.value_type()); // 3 × i64
        Self::encode_leb128(&mut body, 2); body.push(self.target.value_type()); // 2 × i64
        Self::encode_leb128(&mut body, 1); body.push(self.target.value_type()); // 1 × i64
        // (共3+2+1=6个i64局部变量)
        Self::encode_leb128(&mut body, 4);
        body.push(self.target.value_type()); // 4 × i64
        Self::encode_leb128(&mut body, 2);
        body.push(self.target.value_type()); // 2 × i64
        Self::encode_leb128(&mut body, 1);
        body.push(self.target.value_type()); // 1 × i64
        // actually we need 6 total local groups, let me simplify:
        let mut body = Vec::new();
        Self::encode_leb128(&mut body, 1); // 1组局部变量
        Self::encode_leb128(&mut body, 6); // 6个i64
        body.push(self.target.value_type());

        // 编译操作码
        for (op, operands) in &ib.instructions {
            Self::compile_opcode(&mut body, op, operands);
        }

        body.push(WASM_END);
        Self::encode_leb128(&mut section, body.len() as u64);
        section.extend(body);
        Self::encode_leb128(buf, section.len() as u64);
        buf.extend(section);
    }
}

const WASM_NOP: u8 = 0x01;

// ======================================================================
// 第二部分: 蚕蛹寄生壳运行时
// ======================================================================

/// 寄生壳状态 — 脱壳→选壳→穿衣→挂载的生命周期
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellPhase {
    Cocoon,       // 蚕蛹(初始包, 内含解析器和壳选项)
    Peeled,       // 脱壳(已暴露内核)
    Selected,     // 选壳(已选定寄生壳)
    Dressed,      // 穿衣(壳已穿好, 配置生成)
    Mounted,      // 挂载(内核接管, 外部系统反向模仿)
    Failed,       // 失败
}

impl fmt::Display for ShellPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ShellPhase::Cocoon => "蚕蛹",
            ShellPhase::Peeled => "脱壳",
            ShellPhase::Selected => "选壳",
            ShellPhase::Dressed => "穿衣",
            ShellPhase::Mounted => "挂载",
            ShellPhase::Failed => "失败",
        };
        write!(f, "{s}")
    }
}

/// 寄生壳选项 — 可扩展的宿主类型
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellKind {
    LinuxService,     // systemd / init.d
    AndroidIME,       // 输入法服务
    WasmSandbox,      // WASM沙盒容器
    BrowserExt,       // 浏览器扩展
    DesktopApp,       // 桌面应用壳
    Custom(String),   // 自定义
}

/// 寄生壳配置 — 选定后的生成物(配置即代码)
#[derive(Clone, Debug)]
pub struct ShellConfig {
    pub kind: ShellKind,
    pub config_text: String,
    pub mount_point: String,
    pub kernel_path: String,
    pub auto_start: bool,
    pub heartbeat_interval_ms: u64,
}

/// 寄生壳 — 蚕蛹→选壳→穿衣→挂载的完整流程
#[derive(Clone, Debug)]
pub struct ParasiteShell {
    pub phase: ShellPhase,
    pub shell_name: String,
    pub config: Option<ShellConfig>,
    pub available_shells: Vec<ShellKind>,
    pub selected_kind: Option<ShellKind>,  // 选定的壳种类
    pub last_heartbeat_ms: u64,
    pub auto_heartbeats: bool,
    pub mimic_mode: bool,  // 反向模仿模式(外部系统模仿自身)
}

impl ParasiteShell {
    pub fn new(name: &str) -> Self {
        Self {
            phase: ShellPhase::Cocoon,
            shell_name: name.to_string(),
            config: None,
            available_shells: vec![
                ShellKind::LinuxService,
                ShellKind::AndroidIME,
                ShellKind::WasmSandbox,
                ShellKind::BrowserExt,
                ShellKind::DesktopApp,
            ],
            selected_kind: None,
            last_heartbeat_ms: 0,
            auto_heartbeats: false,
            mimic_mode: true,
        }
    }

    /// 注册可用壳选项
    pub fn register_shell(&mut self, kind: ShellKind) {
        if !self.available_shells.contains(&kind) {
            self.available_shells.push(kind);
        }
    }

    /// 获取可用壳数量
    pub fn shell_count(&self) -> usize { self.available_shells.len() }

    /// 步骤1: 脱壳 — 剥开蚕蛹, 暴露内核
    pub fn peel(&mut self) -> bool {
        if self.phase != ShellPhase::Cocoon { return false; }
        self.phase = ShellPhase::Peeled;
        true
    }

    /// 步骤2: 选壳 — 选定寄生壳
    pub fn select_shell(&mut self, kind: &ShellKind) -> bool {
        if self.phase != ShellPhase::Peeled { return false; }
        if !self.available_shells.contains(kind) { return false; }
        self.selected_kind = Some(kind.clone());
        self.phase = ShellPhase::Selected;
        true
    }

    /// 步骤3: 穿衣 — 生成配置文件
    pub fn dress(&mut self) -> bool {
        if self.phase != ShellPhase::Selected { return false; }
        let kind = self.selected_kind.clone().or_else(|| self.available_shells.first().cloned());
        if kind.is_none() { return false; }
        let kind = kind.unwrap();
        self.config = Some(self.generate_config(&kind));
        self.phase = ShellPhase::Dressed;
        true
    }

    /// 步骤4: 挂载 — 接管内核
    pub fn mount(&mut self) -> bool {
        if self.phase != ShellPhase::Dressed { return false; }
        self.phase = ShellPhase::Mounted;
        self.last_heartbeat_ms = current_time_ms();
        true
    }

    /// 一键寄生 — 完整流程
    pub fn parasitize(&mut self, kind: Option<&ShellKind>) -> bool {
        if !self.peel() { return false; }
        match kind {
            Some(k) => { if !self.select_shell(k) { return false; } }
            None => {
                let first = self.available_shells.first().cloned();
                if let Some(f) = first {
                    if !self.select_shell(&f) { return false; }
                } else { return false; }
            }
        }
        if !self.dress() { return false; }
        self.mount()
    }

    /// 发送心跳
    pub fn heartbeat(&mut self) {
        if self.phase == ShellPhase::Mounted {
            self.last_heartbeat_ms = current_time_ms();
        }
    }

    /// 检查心跳是否存活
    pub fn is_alive(&self, timeout_ms: u64) -> bool {
        if self.phase != ShellPhase::Mounted { return false; }
        let now = current_time_ms();
        now - self.last_heartbeat_ms <= timeout_ms
    }

    /// 启用自动心跳
    pub fn enable_auto_heartbeat(&mut self, _interval_ms: u64) {
        self.auto_heartbeats = true;
    }

    /// 禁用自动心跳
    pub fn disable_auto_heartbeat(&mut self) {
        self.auto_heartbeats = false;
    }

    /// 模仿模式 — 外部系统模仿自身
    pub fn set_mimic_mode(&mut self, mimic: bool) {
        self.mimic_mode = mimic;
    }

    /// 卸载壳(恢复)
    pub fn unmount(&mut self) -> bool {
        if self.phase != ShellPhase::Mounted { return false; }
        self.phase = ShellPhase::Dressed;
        self.auto_heartbeats = false;
        self.selected_kind = None;
        true
    }

    /// 生成壳配置 (配置即代码)
    fn generate_config(&self, kind: &ShellKind) -> ShellConfig {
        let config_text = match kind {
            ShellKind::LinuxService => format!(
                "[Unit]\nDescription=寄生壳::{} \nAfter=network.target\n\n[Service]\nType=simple\nExecStart=/usr/local/bin/{} run\nRestart=always\nRestartSec=5\n\n[Install]\nWantedBy=multi-user.target\n",
                self.shell_name, self.shell_name
            ),
            ShellKind::AndroidIME => format!(
                "package com.parasite.{};\n\npublic class {}IME extends InputMethodService {{\n    @Override\n    public void onCreate() {{}}\n    @Override\n    public View onCreateInputView() {{ return null; }}\n}}\n",
                self.shell_name, self.shell_name
            ),
            ShellKind::WasmSandbox => format!(
                "{{\"name\": \"{}\", \"module\": \"{}.wasm\", \"memory_pages\": 16, \"exports\": [\"bag_{}\"]}}\n",
                self.shell_name, self.shell_name, 1
            ),
            ShellKind::BrowserExt => format!(
                "{{\"manifest_version\": 3, \"name\": \"Parasite Shell\", \"version\": \"1.0\", \"permissions\": [\"storage\"], \"host_permissions\": []}}\n"
            ),
            ShellKind::DesktopApp => format!(
                "{{\"name\": \"{}\", \"window\": {{\"width\": 800, \"height\": 600}}, \"entry\": \"main.wasm\"}}\n",
                self.shell_name
            ),
            ShellKind::Custom(name) => format!(
                "{{\"custom\": \"{}\", \"shell_name\": \"{}\"}}\n", name, self.shell_name
            ),
        };

        let mount_point = match kind {
            ShellKind::LinuxService => { format!("/etc/systemd/system/{}.service", self.shell_name) }
            ShellKind::AndroidIME => { format!("/system/inputmethod/{}", self.shell_name) }
            ShellKind::WasmSandbox => { format!("/run/parasite/wasm/{}", self.shell_name) }
            ShellKind::BrowserExt => { format!("/opt/parasite/ext/{}", self.shell_name) }
            ShellKind::DesktopApp => { format!("/usr/share/parasite/{}", self.shell_name) }
            ShellKind::Custom(name) => { format!("/opt/parasite/custom/{}", name) }
        };

        ShellConfig {
            kind: kind.clone(),
            config_text,
            mount_point,
            kernel_path: format!("/parasite/kernel/{}", self.shell_name),
            auto_start: true,
            heartbeat_interval_ms: 5000,
        }
    }

    /// 获取壳阶段
    pub fn phase(&self) -> ShellPhase { self.phase }
}

// ======================================================================
// 第三部分: 寄生管理器 — 多壳并行 + 迁移 + 健康
// ======================================================================

/// 寄生管理器 — 多壳并行管理 · 心跳恢复 · 跨宿主迁移
#[derive(Clone, Debug)]
pub struct ParasiteManager {
    pub shells: HashMap<String, ParasiteShell>,
    pub wasm_compiler: WasmCompiler,
    pub active_count: u64,
    pub total_parasitized: u64,
    pub total_migrated: u64,
}

impl ParasiteManager {
    pub fn new() -> Self {
        Self {
            shells: HashMap::new(),
            wasm_compiler: WasmCompiler::new_dual(),
            active_count: 0,
            total_parasitized: 0,
            total_migrated: 0,
        }
    }

    /// 添加蚕蛹壳
    pub fn add_cocoon(&mut self, name: &str) -> bool {
        if self.shells.contains_key(name) { return false; }
        let shell = ParasiteShell::new(name);
        self.shells.insert(name.to_string(), shell);
        true
    }

    /// 获取壳(可变)
    pub fn get_shell_mut(&mut self, name: &str) -> Option<&mut ParasiteShell> {
        self.shells.get_mut(name)
    }

    /// 获取壳(只读)
    pub fn get_shell(&self, name: &str) -> Option<&ParasiteShell> {
        self.shells.get(name)
    }

    /// 对指定壳执行完整寄生流程
    pub fn parasitize_shell(&mut self, name: &str, kind: Option<ShellKind>) -> bool {
        let kind_ref = kind.as_ref();
        if let Some(shell) = self.shells.get_mut(name) {
            let success = shell.parasitize(kind_ref);
            if success {
                self.total_parasitized += 1;
            }
            success
        } else {
            false
        }
    }

    /// 全局心跳检测
    pub fn heartbeat_check(&mut self, timeout_ms: u64) -> Vec<String> {
        let mut dead = Vec::new();
        for (name, shell) in self.shells.iter_mut() {
            if shell.phase() == ShellPhase::Mounted {
                if !shell.is_alive(timeout_ms) {
                    dead.push(name.clone());
                } else {
                    shell.heartbeat();
                }
            }
        }
        dead
    }

    /// 迁移壳到另一个宿主
    pub fn migrate(&mut self, name: &str, new_host: &str) -> bool {
        // 检查并挂载的壳,然后卸载并重新寄生到新宿主
        let shell = match self.shells.get_mut(name) {
            Some(s) if s.phase() == ShellPhase::Mounted => s,
            _ => return false,
        };
        shell.unmount();
        self.total_migrated += 1;

        // 直接跳到Peeled阶段(已经脱过壳了)
        shell.phase = ShellPhase::Peeled;
        shell.selected_kind = None;
        shell.auto_heartbeats = false;

        // 添加自定义壳
        let custom = ShellKind::Custom(new_host.to_string());
        shell.register_shell(custom.clone());

        // 选壳→穿衣→挂载
        if !shell.select_shell(&custom) { return false; }
        if !shell.dress() { return false; }
        shell.mount()
    }

    /// 获取所有已挂载壳的名称
    pub fn mounted_shells(&self) -> Vec<String> {
        self.shells.iter()
            .filter(|(_, s)| s.phase() == ShellPhase::Mounted)
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// 移除壳
    pub fn remove_shell(&mut self, name: &str) -> Option<ParasiteShell> {
        self.shells.remove(name)
    }

    /// 壳数量
    pub fn shell_count(&self) -> usize { self.shells.len() }

    /// 管理器摘要
    pub fn summary(&self) -> String {
        let mounted = self.mounted_shells().len();
        let failed = self.shells.values().filter(|s| s.phase() == ShellPhase::Failed).count();
        format!(
            "寄生管理: {}壳(挂载{} 失败{}) 寄生{}次 迁移{}次",
            self.shells.len(), mounted, failed,
            self.total_parasitized, self.total_migrated,
        )
    }
}

// ======================================================================
// 第四部分: CelestialPerceptionV13 — 编排引擎
// ======================================================================

/// CelestialPerceptionV13 — 天觉v13编排引擎
/// WASM编译目标 + 蚕蛹寄生壳的总控中心
#[derive(Clone, Debug)]
pub struct CelestialPerceptionV13 {
    pub version: u32,
    pub parasite_mgr: ParasiteManager,
    pub wasm32_cache: HashMap<u64, WasmModule>,
    pub wasm64_cache: HashMap<u64, WasmModule>,
    pub active: bool,
}

impl CelestialPerceptionV13 {
    pub fn new() -> Self {
        Self {
            version: V13_VERSION,
            parasite_mgr: ParasiteManager::new(),
            wasm32_cache: HashMap::new(),
            wasm64_cache: HashMap::new(),
            active: true,
        }
    }

    /// 编译袋子指令袋为WASM双目标
    pub fn compile_to_wasm(&mut self, ib: &InstructionBag) -> (WasmModule, WasmModule) {
        let (m32, m64) = self.parasite_mgr.wasm_compiler.compile_dual(ib);
        self.wasm32_cache.insert(ib.source_bag_id, m32.clone());
        self.wasm64_cache.insert(ib.source_bag_id, m64.clone());
        (m32, m64)
    }

    /// 编译单目标
    pub fn compile_single(&mut self, ib: &InstructionBag, target: WasmTarget) -> WasmModule {
        let mut compiler = WasmCompiler::new(target);
        let module = compiler.compile(ib);
        match target {
            WasmTarget::Wasm32 => { self.wasm32_cache.insert(ib.source_bag_id, module.clone()); }
            WasmTarget::Wasm64 => { self.wasm64_cache.insert(ib.source_bag_id, module.clone()); }
        }
        module
    }

    /// 添加寄生壳
    pub fn add_parasite_shell(&mut self, name: &str) -> bool {
        self.parasite_mgr.add_cocoon(name)
    }

    /// 执行完整寄生
    pub fn parasitize(&mut self, shell_name: &str, shell_kind: Option<ShellKind>) -> bool {
        self.parasite_mgr.parasitize_shell(shell_name, shell_kind)
    }

    /// 检查存活壳
    pub fn heartbeat_check(&mut self, timeout_ms: u64) -> Vec<String> {
        self.parasite_mgr.heartbeat_check(timeout_ms)
    }

    /// 免疫状态
    pub fn immunity_report(&self) -> String {
        let mounted = self.parasite_mgr.mounted_shells().len();
        let wasm32_count = self.wasm32_cache.len();
        let wasm64_count = self.wasm64_cache.len();
        format!(
            "天觉v13 Wasm双目标({}+{}) 寄生壳{}/{} 运行中{}",
            wasm32_count, wasm64_count,
            mounted, self.parasite_mgr.shell_count(),
            if self.active { "激活" } else { "休眠" },
        )
    }
}

// ======================================================================
// 辅助函数
// ======================================================================

fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 单元测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v13_engine_creation() {
        let v13 = CelestialPerceptionV13::new();
        assert_eq!(v13.version, V13_VERSION);
        assert!(v13.active);
        assert_eq!(v13.parasite_mgr.shell_count(), 0);
    }

    #[test]
    fn wasm_target_properties() {
        assert_eq!(WasmTarget::Wasm32.pointer_size(), 4);
        assert_eq!(WasmTarget::Wasm64.pointer_size(), 8);
        assert_eq!(format!("{}", WasmTarget::Wasm32), "wasm32");
        assert_eq!(format!("{}", WasmTarget::Wasm64), "wasm64");
    }

    #[test]
    fn wasm_module_creation() {
        let module = WasmModule::new(WasmTarget::Wasm32, 1);
        assert_eq!(module.target, WasmTarget::Wasm32);
        assert_eq!(module.source_bag_id, 1);
        assert_eq!(module.memory_pages, 1);
        let header = module.header();
        // \0asm
        assert_eq!(header[0..4], [0x00, 0x61, 0x73, 0x6D]);
    }

    #[test]
    fn wasm_compiler_compiles_instruction_bag() {
        let mut ib = InstructionBag::new(1);
        ib.emit(OpCode::Spawn, vec![1]);
        ib.emit(OpCode::AddValue, vec![100]);
        ib.emit(OpCode::PassThrough, vec![]);

        let mut compiler = WasmCompiler::new(WasmTarget::Wasm32);
        let module = compiler.compile(&ib);

        assert!(module.has_valid_header());
        assert!(module.len() > 8);
        assert_eq!(compiler.compile_count, 1);
        assert_eq!(module.source_ib_len, 3);
        assert!(module.entry_point.is_some());
    }

    #[test]
    fn wasm_dual_compile_produces_both_targets() {
        let mut ib = InstructionBag::new(42);
        ib.emit(OpCode::LoadBag, vec![42]);
        ib.emit(OpCode::AddValue, vec![500]);

        let mut compiler = WasmCompiler::new_dual();
        let (m32, m64) = compiler.compile_dual(&ib);

        assert_eq!(m32.target, WasmTarget::Wasm32);
        assert_eq!(m64.target, WasmTarget::Wasm64);
        assert!(m32.has_valid_header());
        assert!(m64.has_valid_header());
    }

    #[test]
    fn wasm_module_contains_symbol() {
        let mut ib = InstructionBag::new(7);
        ib.emit(OpCode::Spawn, vec![7]);
        let mut compiler = WasmCompiler::new(WasmTarget::Wasm32);
        let module = compiler.compile(&ib);

        assert!(module.resolve_symbol("bag_7").is_some());
        assert!(module.resolve_symbol("main").is_some());
        assert!(module.resolve_symbol("nonexistent").is_none());
    }

    #[test]
    fn shell_phase_transitions() {
        let mut shell = ParasiteShell::new("test_shell");
        assert_eq!(shell.phase(), ShellPhase::Cocoon);
        assert!(shell.peel());
        assert_eq!(shell.phase(), ShellPhase::Peeled);
        assert!(shell.select_shell(&ShellKind::WasmSandbox));
        assert_eq!(shell.phase(), ShellPhase::Selected);
        assert!(shell.dress());
        assert_eq!(shell.phase(), ShellPhase::Dressed);
        assert!(shell.mount());
        assert_eq!(shell.phase(), ShellPhase::Mounted);
    }

    #[test]
    fn shell_parasitate_full_flow() {
        let mut shell = ParasiteShell::new("parasite1");
        assert!(shell.parasitize(Some(&ShellKind::LinuxService)));
        assert_eq!(shell.phase(), ShellPhase::Mounted);
        assert!(shell.is_alive(10_000));
    }

    #[test]
    fn shell_config_generation() {
        let mut shell = ParasiteShell::new("cfg_test");
        shell.parasitize(Some(&ShellKind::AndroidIME));
        let config = shell.config.as_ref().unwrap();
        assert_eq!(config.kind, ShellKind::AndroidIME);
        assert!(!config.config_text.is_empty());
        assert!(!config.mount_point.is_empty());
        assert!(config.auto_start);
    }

    #[test]
    fn shell_mimic_mode_default_on() {
        let shell = ParasiteShell::new("mimic_test");
        assert!(shell.mimic_mode);  // 默认开启反向模仿
    }

    #[test]
    fn shell_unmount() {
        let mut shell = ParasiteShell::new("umount_test");
        shell.parasitize(Some(&ShellKind::WasmSandbox));
        assert!(shell.unmount());
        assert_eq!(shell.phase(), ShellPhase::Dressed);
        assert!(!shell.auto_heartbeats);
    }

    #[test]
    fn shell_heartbeat_updates_timestamp() {
        let mut shell = ParasiteShell::new("hb_test");
        shell.parasitize(Some(&ShellKind::DesktopApp));
        let before = shell.last_heartbeat_ms;
        shell.heartbeat();
        assert!(shell.last_heartbeat_ms >= before);
        assert!(shell.is_alive(60_000));
    }

    #[test]
    fn parasite_manager_add_and_parasitize() {
        let mut mgr = ParasiteManager::new();
        assert!(mgr.add_cocoon("shell_a"));
        assert_eq!(mgr.shell_count(), 1);
        assert!(mgr.parasitize_shell("shell_a", Some(ShellKind::BrowserExt)));
        assert_eq!(mgr.mounted_shells().len(), 1);
    }

    #[test]
    fn parasite_manager_rejects_duplicate() {
        let mut mgr = ParasiteManager::new();
        assert!(mgr.add_cocoon("dup_shell"));
        assert!(!mgr.add_cocoon("dup_shell")); // 已存在
        assert_eq!(mgr.shell_count(), 1);
    }

    #[test]
    fn parasite_migrate_shell() {
        let mut mgr = ParasiteManager::new();
        mgr.add_cocoon("migrating");
        // 先寄生到一个标准壳
        mgr.parasitize_shell("migrating", Some(ShellKind::WasmSandbox));
        assert_eq!(mgr.mounted_shells().len(), 1);
        // 迁移到新的自定义宿主
        assert!(mgr.migrate("migrating", "new_host_b"));
        // 迁移后仍然是挂载状态
        assert_eq!(mgr.mounted_shells().len(), 1);
    }

    #[test]
    fn v13_compile_to_wasm_stores_in_cache() {
        let mut v13 = CelestialPerceptionV13::new();
        let mut ib = InstructionBag::new(100);
        ib.emit(OpCode::Spawn, vec![100]);
        ib.emit(OpCode::AddValue, vec![800]);

        v13.compile_to_wasm(&ib);
        assert!(v13.wasm32_cache.contains_key(&100));
        assert!(v13.wasm64_cache.contains_key(&100));
    }

    #[test]
    fn v13_full_flow() {
        // 完整流程: 创建壳→寄生→编译WASM→检测心跳
        let mut v13 = CelestialPerceptionV13::new();
        assert!(v13.add_parasite_shell("core"));
        assert!(v13.parasitize("core", Some(ShellKind::WasmSandbox)));

        let mut ib = InstructionBag::new(1);
        ib.emit(OpCode::Spawn, vec![1]);
        ib.emit(OpCode::AddValue, vec![1000]);
        let (m32, m64) = v13.compile_to_wasm(&ib);
        assert!(m32.has_valid_header());
        assert!(m64.has_valid_header());

        let dead = v13.heartbeat_check(60_000);
        assert!(dead.is_empty()); // 正常壳应该存活

        let report = v13.immunity_report();
        assert!(report.contains("寄生壳1/1"));
    }

    #[test]
    fn shell_phase_display() {
        assert_eq!(format!("{}", ShellPhase::Cocoon), "蚕蛹");
        assert_eq!(format!("{}", ShellPhase::Mounted), "挂载");
    }

    #[test]
    fn v13_compile_single_target() {
        let mut v13 = CelestialPerceptionV13::new();
        let mut ib = InstructionBag::new(200);
        ib.emit(OpCode::Spawn, vec![200]);

        let module = v13.compile_single(&ib, WasmTarget::Wasm64);
        assert_eq!(module.target, WasmTarget::Wasm64);
        assert!(v13.wasm64_cache.contains_key(&200));
    }
}
