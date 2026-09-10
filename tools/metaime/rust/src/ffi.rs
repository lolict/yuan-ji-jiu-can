//! MetaIME · FFI 胶水层 (暴露 C-ABI 给 Android JNI)
//!
//! 此模块让 C/C++ 侧 (jni_bridge.cpp) 能调用 Rust 七层编码引擎
//! 所有函数使用 extern "C" 保证 ABI 兼容
//!
//! 注意:
//!   - 编译目标: staticlib + cdylib (生成 libmetaime_core.so)
//!   - 零堆分配主要用于 no_std 配置；FFI 入口因需跨语言传递字符串，使用 Box<String>

use std::ffi::{CStr, CString, c_void, c_char, c_int};
use std::ptr;

use crate::radix3d::Radix3DInterpreter;
use crate::translator::FQMTranslator;

// ============================================================
// Opaque handle 定义
// ============================================================

pub struct MetaEngine {
    translator: FQMTranslator,
    radix: Radix3DInterpreter,
    state: u8,          // 当前 L1..L7 层
    buffer: [u8; 64],   // 输入缓冲 (固定大小)
    buf_len: usize,
    mode: [u8; 16],     // 当前模式名 (pinyin/command/name)
    mode_len: usize,
}

// ============================================================
// FFI 入口点 ( JNI 调用 )
// ============================================================

#[no_mangle]
pub extern "C" fn metaime_create() -> *mut c_void {
    let engine = Box::new(MetaEngine {
        translator: FQMTranslator::new(),
        radix: Radix3DInterpreter::new(),
        state: 1,
        buffer: [0u8; 64],
        buf_len: 0,
        mode: [0u8; 16],
        mode_len: 0,
    });
    Box::into_raw(engine) as *mut c_void
}

#[no_mangle]
pub extern "C" fn metaime_destroy(handle: *mut c_void) {
    if handle.is_null() { return; }
    unsafe { drop(Box::from_raw(handle as *mut MetaEngine)); }
}

#[no_mangle]
pub extern "C" fn metaime_push_key(handle: *mut c_void, key_code: c_int) -> *const c_char {
    if handle.is_null() { return ptr::null(); }
    let engine = unsafe { &mut *(handle as *mut MetaEngine) };
    let result = engine.push_key(key_code as u8);
    match CString::new(result) {
        Ok(cs) => cs.into_raw(),
        Err(_) => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn metaime_reset(handle: *mut c_void) -> *const c_char {
    if handle.is_null() { return ptr::null(); }
    let engine = unsafe { &mut *(handle as *mut MetaEngine) };
    let result = engine.reset();
    match CString::new(result) {
        Ok(cs) => cs.into_raw(),
        Err(_) => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn metaime_switch_mode(handle: *mut c_void, mode: *const c_char) -> *const c_char {
    if handle.is_null() || mode.is_null() { return ptr::null(); }
    let engine = unsafe { &mut *(handle as *mut MetaEngine) };
    let mode_str = unsafe { CStr::from_ptr(mode) }.to_string_lossy().to_string();
    let result = engine.switch_mode(&mode_str);
    match CString::new(result) {
        Ok(cs) => cs.into_raw(),
        Err(_) => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn metaime_get_stats(handle: *mut c_void) -> *const c_char {
    if handle.is_null() { return ptr::null(); }
    let engine = unsafe { &*(handle as *mut MetaEngine) };
    let stats = format!("{{\"state\":{},\"buf_len\":{},\"mode\":\"{}\"}}",
                        engine.state, engine.buf_len,
                        std::str::from_utf8(&engine.mode[..engine.mode_len]).unwrap_or("?"));
    match CString::new(stats) {
        Ok(cs) => cs.into_raw(),
        Err(_) => ptr::null(),
    }
}

// ============================================================
// MetaEngine 方法
// ============================================================

impl MetaEngine {
    fn push_key(&mut self, key_code: u8) -> String {
        let ch = match key_code {
            29..=54 => (b'a' + key_code - 29) as char,  // a-z
            7..=16  => (b'0' + key_code - 7) as char,   // 0-9
            111 => 27 as char,  // ESC 清空
            _ => '?' as char,
        };

        if (ch as u8) == 27 {
            return self.reset();
        }

        if self.buf_len < self.buffer.len() {
            self.buffer[self.buf_len] = ch as u8;
            self.buf_len += 1;
        }

        let input = std::str::from_utf8(&self.buffer[..self.buf_len]).unwrap_or("");
        self.classify(input)
    }

    fn classify(&self, input: &str) -> String {
        // 短路径: 命令模式
        if input == "mlll" || input == "njsg" || input == "mltt" {
            return format!("{{\"intent\":\"COMMAND\",\"payload\":\"{}\"}}", input);
        }
        // L5+L6: 匹配 外来符号表
        if let Some(path) = self.translator.translate_path(input) {
            return format!("{{\"intent\":\"INPUT\",\"payload\":\"{}\",\"semantic\":\"{}\"}}", input, path);
        }
        // 普通字符上屏
        format!("{{\"intent\":\"INPUT\",\"payload\":\"{}\"}}", input)
    }

    fn reset(&mut self) -> String {
        self.state = 1;
        self.buf_len = 0;
        self.buffer = [0u8; 64];
        "{{\"intent\":\"CLEAR\",\"payload\":\"\"}}".to_string()
    }

    fn switch_mode(&mut self, mode: &str) -> String {
        for (i, b) in mode.bytes().enumerate() {
            if i < self.mode.len() { self.mode[i] = b; }
        }
        self.mode_len = mode.len().min(self.mode.len());
        self.state = 1;
        self.buf_len = 0;
        format!("{{\"intent\":\"TRANSFORM\",\"payload\":\"{}\"}}", mode)
    }
}
