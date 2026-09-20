// hunyuan-wasm · 满全法容器注册系统 · WASM核心
// 提供:进制寻址 + 容器注册 + 公路链接 + 可达性计算
// 无外部依赖 · 纯Rust核心 · JS负责Canvas绘制

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::slice;

// ── 全球状态(单例) ──
static mut REGISTRY: Option<Registry> = None;

fn get_reg<'a>() -> &'a mut Registry {
    unsafe {
        if REGISTRY.is_none() {
            REGISTRY = Some(Registry::new());
        }
        REGISTRY.as_mut().unwrap()
    }
}

// ── 容器 ──
#[derive(Clone)]
pub struct Entry {
    pub addr: u32,       // 进制地址
    pub ns: String,      // 命名空间
    pub state: u8,       // 工作状态
    pub version: u32,
    pub family: u8,      // 0-4 族
    pub structure: u8,   // 0-7 结构
    pub polarity: u8,    // 0=阴 1=阳
}

// ── 注册清单 ──
pub struct Registry {
    pub entries: Vec<Entry>,
    pub links: Vec<(u32, u32)>,
}

impl Registry {
    pub fn new() -> Self {
        Self { entries: Vec::new(), links: Vec::new() }
    }

    pub fn add(&mut self, addr: u32, ns: &str, state: u8) {
        // 计算族/结构/极性 从地址本身
        let family = ((addr >> 12) % 5) as u8;
        let structure = ((addr >> 8) % 8) as u8;
        let polarity = (addr >> 15) as u8 & 1;

        for e in &mut self.entries {
            if e.addr == addr {
                e.state = state;
                e.version += 1;
                e.ns = ns.to_string();
                return;
            }
        }
        self.entries.push(Entry {
            addr, ns: ns.to_string(), state, version: 1,
            family, structure, polarity,
        });
        self.entries.sort_by_key(|e| e.addr);
    }

    pub fn link(&mut self, a: u32, b: u32) { self.links.push((a, b)); }

    pub fn find(&self, addr: u32) -> Option<&Entry> {
        self.entries.iter().find(|e| e.addr == addr)
    }

    pub fn reachable(&self, from: u32) -> Vec<u32> {
        let mut reach = vec![from];
        let mut changed = true;
        while changed {
            changed = false;
            let snapshot = reach.clone();
            for node in &snapshot {
                for (a, b) in &self.links {
                    if a == node && !reach.contains(b) { reach.push(*b); changed = true; }
                    if b == node && !reach.contains(a) { reach.push(*a); changed = true; }
                }
            }
        }
        reach.sort();
        reach
    }

    /// 重置为初始种子状态
    pub fn seed(&mut self) {
        self.entries.clear();
        self.links.clear();
        self.add(0x0101, "声母/Shengmu", 1);
        self.add(0x0102, "韵母/Yunmu", 1);
        self.add(0x0103, "声调/Shengdiao", 0);
        self.add(0x0210, "容器/Container", 2);
        self.add(0x0211, "边界/Boundary", 0);
        self.add(0x0212, "夫妻/Couple", 0);
        self.add(0x0320, "网状/Web", 1);
        self.add(0x0001, "中性/Neutral", 0);
        self.link(0x0101, 0x0102);
        self.link(0x0102, 0x0103);
        self.link(0x0210, 0x0211);
        self.link(0x0211, 0x0212);
        self.link(0x0212, 0x0320);
        self.link(0x0103, 0x0210);
    }
}

// ── FFI接口 ──

#[no_mangle]
pub extern "C" fn alloc_mem(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        alloc(layout)
    }
}

#[no_mangle]
pub extern "C" fn free_mem(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        dealloc(ptr, layout);
    }
}

#[no_mangle]
pub extern "C" fn reg_seed() {
    get_reg().seed();
}

#[no_mangle]
pub extern "C" fn reg_add(addr: u32, state: u8, ns: *const c_char) -> u8 {
    if ns.is_null() { return 0; }
    let ns_str = unsafe { CStr::from_ptr(ns) }.to_string_lossy();
    get_reg().add(addr, &ns_str, state);
    1
}

#[no_mangle]
pub extern "C" fn reg_link(a: u32, b: u32) {
    get_reg().link(a, b);
}

/// 返回注册清单JSON
#[no_mangle]
pub extern "C" fn reg_dump() -> *mut c_char {
    let reg = get_reg();
    let mut parts = Vec::new();
    for e in &reg.entries {
        parts.push(format!(
            r#"{{"addr":{},"ns":"{}","state":{},"v":{},"fam":{},"st":{},"pol":{}}}"#,
            e.addr, e.ns, e.state, e.version, e.family, e.structure, e.polarity
        ));
    }
    let json = format!("[{}]", parts.join(","));
    CString::new(json).unwrap().into_raw()
}

/// 返回公路链接JSON
#[no_mangle]
pub extern "C" fn reg_links() -> *mut c_char {
    let reg = get_reg();
    let parts: Vec<String> = reg.links.iter().map(|(a,b)| format!(r#"{{"a":{},"b":{}}}"#, a, b)).collect();
    let json = format!("[{}]", parts.join(","));
    CString::new(json).unwrap().into_raw()
}

/// 返回某地址的可达集合JSON
#[no_mangle]
pub extern "C" fn reg_reach(addr: u32) -> *mut c_char {
    let reach = get_reg().reachable(addr);
    let parts: Vec<String> = reach.iter().map(|a| format!("{}", a)).collect();
    let json = format!("[{}]", parts.join(","));
    CString::new(json).unwrap().into_raw()
}

/// 释放字符串内存
#[no_mangle]
pub extern "C" fn reg_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)); }
    }
}

/// 返回第一条JSON字段长度(用于JS知道分配多少内存)
#[no_mangle]
pub extern "C" fn reg_dump_len() -> usize {
    let reg = get_reg();
    let mut len = 2; // []
    for e in &reg.entries {
        len += 70; // 粗略估算每条entry JSON长度
    }
    len
}

/// 进制位展开: 数值 -> 各位
#[no_mangle]
pub extern "C" fn fq_digits(value: u64, out: *mut u64) -> u32 {
    if out.is_null() { return 0; }
    let bases: [u64; 6] = [48, 64, 16, 8, 5, 2];
    let mut rem = value;
    let mut count = 0;
    unsafe {
        for i in 0..6 {
            *out.add(i) = rem % bases[i];
            rem /= bases[i];
            count += 1;
        }
        if rem > 0 { *out.add(6) = rem; count += 1; }
    }
    count
}

/// 进制位合成: 各位 -> 数值(最多支持8位,第7+位为超出基底的高位纯累加)
#[no_mangle]
pub extern "C" fn fq_compose(digits: *const u64, count: u32) -> u64 {
    if digits.is_null() || count == 0 { return 0; }
    let bases: [u64; 6] = [48, 64, 16, 8, 5, 2];
    let mut result = 0u64;
    let mut weight = 1u64;
    unsafe {
        let n = count.min(8);
        for i in 0..n as usize {
            let digit = *digits.add(i);
            if i < 6 {
                result += digit * weight;
                weight *= bases[i];
            } else {
                // 超出基底的高位:继续递增权重
                result += digit * weight;
                weight = weight.saturating_mul(2);
            }
        }
    }
    result
}
