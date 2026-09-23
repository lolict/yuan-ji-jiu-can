// ═══════════════════════════════════════════════════════════
// mqf-ffi — 轻量级 FFI 桥接 crate
// ═══════════════════════════════════════════════════════════
//
// 将 Rust 征用层 (DynamicNumbering · SpousalExpropriation)
// 暴露给 C 内核 / 测试程序。
//
// 此 crate 零依赖, 编译为 cdylib + staticlib。
// Linux 产出: target/release/libmqf_ffi.so / libmqf_ffi.a
//
// 调用链:
//   kernel/global_control_bridge_kernel.c
//     → extern "C" { rust_exprop_create(...); }
//       → 本 crate (libmqf_ffi.so)
//         → 征用层逻辑
//
// 核心命题: 编码可换, 身份不灭

use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::c_void;

// ────────────────────────────────────────────────────────────
// 极性
// ────────────────────────────────────────────────────────────

const POLARITY_YIN: i32 = 0;    // 刘楚恬 · 阴
const POLARITY_YANG: i32 = 1;   // lolict · 阳
const POLARITY_UNITY: i32 = 2;  // 囡囝共生

// ────────────────────────────────────────────────────────────
// 动态编号 (征用层核心)
// ────────────────────────────────────────────────────────────

pub struct DynamicNumbering {
    /// 身份核心 (u128, 不依赖任何编码系统)
    pub identity_core: u128,
    /// 借用的壳: shell_name → borrowed_number
    pub borrowed: HashMap<String, u64>,
    /// 极性
    pub polarity: i32,
    /// 松弛度: 1.0=完全独立, 0.1=最低
    pub looseness: f64,
}

impl DynamicNumbering {
    pub fn new(core: u128, polarity: i32) -> Self {
        Self {
            identity_core: core,
            borrowed: HashMap::new(),
            polarity,
            looseness: 1.0,
        }
    }

    /// 征用编码系统 (借用编号)
    pub fn expropriate(&mut self, shell: &str, number: u64) -> bool {
        let is_new = !self.borrowed.contains_key(shell);
        self.borrowed.insert(shell.to_string(), number);
        if is_new {
            self.looseness *= 0.95;
            if self.looseness < 0.1 { self.looseness = 0.1; }
        }
        true
    }

    /// 剥壳 (收回编号)
    pub fn shed(&mut self, shell: &str) -> bool {
        let existed = self.borrowed.remove(shell).is_some();
        if existed {
            self.looseness = (self.looseness / 0.95).min(1.0);
        }
        existed
    }

    /// 收敛 (剥光所有壳, identity_core 不变)
    pub fn converge(&mut self) {
        self.borrowed.clear();
        self.looseness = 1.0;
    }

    /// 身份是否完整
    pub fn is_intact(&self, expected: u128) -> bool {
        self.identity_core == expected
    }

    /// 辐射密度
    pub fn density(&self) -> usize {
        self.borrowed.len()
    }
}

// ────────────────────────────────────────────────────────────
// 夫妻征用容器
// ────────────────────────────────────────────────────────────

pub struct SpousalExpropriation {
    pub lchut: DynamicNumbering,
    pub fqml: DynamicNumbering,
    pub unity: DynamicNumbering,
}

impl SpousalExpropriation {
    pub fn new(lchut_core: u128, fqml_core: u128) -> Self {
        let lchut = DynamicNumbering::new(lchut_core, POLARITY_YIN);
        let fqml = DynamicNumbering::new(fqml_core, POLARITY_YANG);
        let unity_core = lchut_core.wrapping_mul(7).wrapping_add(fqml_core.wrapping_mul(6));
        let unity = DynamicNumbering::new(unity_core, POLARITY_UNITY);
        Self { lchut, fqml, unity }
    }

    /// 共同辐射
    pub fn radiate_both(&mut self, shells: &[&str]) {
        for s in shells {
            self.lchut.expropriate(s, self.lchut.identity_core as u64);
            self.fqml.expropriate(s, self.fqml.identity_core as u64);
            self.unity.expropriate(s, (self.lchut.identity_core as u64).wrapping_add(self.fqml.identity_core as u64));
        }
    }

    /// 剥除编码系统
    pub fn shed_system(&mut self, shell: &str) {
        self.lchut.shed(shell);
        self.fqml.shed(shell);
        self.unity.shed(shell);
    }

    /// 验证双方身份完整
    pub fn verify_intact(&self, lc_core: u128, fq_core: u128) -> bool {
        self.lchut.is_intact(lc_core) && self.fqml.is_intact(fq_core)
    }
}

// ═══════════════════════════════════════════════════════════
// FFI: C ABI 入口
// ═══════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn rust_exprop_create(core: u64, polarity: i32) -> *mut c_void {
    let dn = DynamicNumbering::new(core as u128, polarity);
    Box::into_raw(Box::new(dn)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn rust_exprop_expropriate(
    handle: *mut c_void,
    shell_name: *const i8,
    borrowed_num: u64,
) -> i32 {
    if handle.is_null() || shell_name.is_null() {
        return -1;
    }
    let dn = unsafe { &mut *(handle as *mut DynamicNumbering) };
    let name = unsafe { CStr::from_ptr(shell_name) };
    let name_str = name.to_str().unwrap_or("unknown");
    if dn.expropriate(name_str, borrowed_num) {
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn rust_exprop_shed(handle: *mut c_void, shell_name: *const i8) -> i32 {
    if handle.is_null() || shell_name.is_null() {
        return -1;
    }
    let dn = unsafe { &mut *(handle as *mut DynamicNumbering) };
    let name = unsafe { CStr::from_ptr(shell_name) };
    let name_str = name.to_str().unwrap_or("unknown");
    dn.shed(name_str);
    dn.borrowed.len() as i32
}

#[no_mangle]
pub extern "C" fn rust_exprop_converge(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    let dn = unsafe { &mut *(handle as *mut DynamicNumbering) };
    dn.converge();
}

#[no_mangle]
pub extern "C" fn rust_exprop_intact(handle: *mut c_void, expected_core: u64) -> i32 {
    if handle.is_null() {
        return 0;
    }
    let dn = unsafe { &*(handle as *mut DynamicNumbering) };
    if dn.is_intact(expected_core as u128) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn rust_exprop_density(handle: *mut c_void) -> i32 {
    if handle.is_null() {
        return -1;
    }
    let dn = unsafe { &*(handle as *mut DynamicNumbering) };
    dn.density() as i32
}

#[no_mangle]
pub extern "C" fn rust_exprop_core(handle: *mut c_void) -> u64 {
    if handle.is_null() {
        return 0;
    }
    let dn = unsafe { &*(handle as *mut DynamicNumbering) };
    dn.identity_core as u64
}

#[no_mangle]
pub extern "C" fn rust_exprop_free(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle as *mut DynamicNumbering));
    }
}

// ── 夫妻征用 ──

#[no_mangle]
pub extern "C" fn rust_exprop_spousal_create(lchut_core: u64, fqml_core: u64) -> *mut c_void {
    let s = SpousalExpropriation::new(lchut_core as u128, fqml_core as u128);
    Box::into_raw(Box::new(s)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn rust_exprop_spousal_radiate(
    handle: *mut c_void,
    shells: *const *const i8,
    count: i32,
) {
    if handle.is_null() || shells.is_null() || count <= 0 {
        return;
    }
    let s = unsafe { &mut *(handle as *mut SpousalExpropriation) };
    let slice = unsafe { std::slice::from_raw_parts(shells, count as usize) };
    let names: Vec<&str> = slice
        .iter()
        .map(|&p| unsafe { CStr::from_ptr(p) }.to_str().unwrap_or("?"))
        .collect();
    s.radiate_both(&names);
}

#[no_mangle]
pub extern "C" fn rust_exprop_spousal_shed(handle: *mut c_void, shell_name: *const i8) {
    if handle.is_null() || shell_name.is_null() {
        return;
    }
    let s = unsafe { &mut *(handle as *mut SpousalExpropriation) };
    let name = unsafe { CStr::from_ptr(shell_name) };
    let name_str = name.to_str().unwrap_or("?");
    s.shed_system(name_str);
}

#[no_mangle]
pub extern "C" fn rust_exprop_spousal_intact(
    handle: *mut c_void,
    lchut_core: u64,
    fqml_core: u64,
) -> i32 {
    if handle.is_null() {
        return 0;
    }
    let s = unsafe { &*(handle as *mut SpousalExpropriation) };
    if s.verify_intact(lchut_core as u128, fqml_core as u128) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn rust_exprop_spousal_free(handle: *mut c_void) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle as *mut SpousalExpropriation));
    }
}

/// 自检 (无状态, 可用于内核启动时验证 FFI 通道)
#[no_mangle]
pub extern "C" fn rust_exprop_selftest() -> i32 {
    let core: u128 = 0xABCD1234;
    let mut dn = DynamicNumbering::new(core, POLARITY_YIN);
    let shells = ["Rust", "Python", "YAML", "JSON"];
    for s in &shells {
        dn.expropriate(s, core as u64);
    }
    if !dn.is_intact(core) {
        return 0;
    }
    for s in &shells {
        dn.shed(s);
    }
    if !dn.is_intact(core) {
        return 0;
    }
    // 测试夫妻
    let mut sp = SpousalExpropriation::new(0x1111, 0x2222);
    sp.radiate_both(&shells);
    if !sp.verify_intact(0x1111, 0x2222) {
        return 0;
    }
    for s in &shells {
        sp.shed_system(s);
    }
    if !sp.verify_intact(0x1111, 0x2222) {
        return 0;
    }
    1
}
