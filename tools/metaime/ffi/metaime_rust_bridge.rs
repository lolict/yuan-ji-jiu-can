//! MetaIME Rust FFI Bridge · 导出 Rust 实现为 C ABI
//!
//! 编译: cargo build --release
//! 生成: libmetaime_rust_bridge.so

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

// 引入 Rust 实现模块
use metaime::constants::*;
use metaime::loader::FQMLoader;
use metaime::translator::FQMTranslator;

/// 错误码
#[repr(C)]
pub enum MiErr {
    Ok = 0,
    NotFound = -1,
    Full = -2,
    Invalid = -3,
    Overflow = -4,
}

/// 本体论路径
#[repr(C)]
pub struct MiPath {
    domain: [c_char; 24],
    identity: [c_char; 24],
}

/// 符号条目
#[repr(C)]
pub struct MiSymbol {
    symbol: [c_char; 8],
    position: u8,
}

/// 容器
#[repr(C)]
pub struct MiContainer {
    name: [c_char; 24],
    symbols: [MiSymbol; 64],
    symbol_count: u8,
    capacity: u8,
    polarity: u8,
}

/// 加载器句柄
pub struct MiLoader {
    inner: FQMLoader,
}

#[no_mangle]
pub extern "C" fn mi_loader_create() -> *mut MiLoader {
    let loader = FQMLoader::with_defaults();
    Box::into_raw(Box::new(MiLoader { inner: loader }))
}

#[no_mangle]
pub unsafe extern "C" fn mi_loader_destroy(l: *mut MiLoader) {
    if !l.is_null() {
        drop(Box::from_raw(l));
    }
}

#[no_mangle]
pub unsafe extern "C" fn mi_loader_load_default(l: *mut MiLoader) -> MiErr {
    if l.is_null() {
        return MiErr::Invalid;
    }
    MiErr::Ok
}

#[no_mangle]
pub unsafe extern "C" fn mi_loader_container_count(l: *const MiLoader) -> i32 {
    if l.is_null() {
        return 0;
    }
    (*l).inner.container_count as i32
}

#[no_mangle]
pub unsafe extern "C" fn mi_loader_get_container(
    l: *const MiLoader,
    name: *const c_char,
    out: *mut MiContainer,
) -> MiErr {
    if l.is_null() || name.is_null() || out.is_null() {
        return MiErr::Invalid;
    }

    let name_str = match CStr::from_ptr(name).to_str() {
        Ok(s) => s,
        Err(_) => return MiErr::Invalid,
    };

    let loader = &(*l).inner;

    if let Some(container) = loader.get_container_by_type(metaime::loader::ContainerType::ShengMu) {
        if name_str == "声母" {
            let name_bytes = "声母".as_bytes();
            for (i, &b) in name_bytes.iter().enumerate() {
                (*out).name[i] = b as c_char;
            }
            (*out).name[name_bytes.len()] = 0;
            (*out).symbol_count = container.symbol_count as u8;
            (*out).capacity = container.capacity as u8;

            for (i, &sym) in container.symbols[..container.symbol_count].iter().enumerate() {
                let sym_bytes = sym.as_bytes();
                for (j, &b) in sym_bytes.iter().enumerate() {
                    (*out).symbols[i].symbol[j] = b as c_char;
                }
                (*out).symbols[i].symbol[sym_bytes.len()] = 0;
                (*out).symbols[i].position = container.positions[i] as u8;
            }
            return MiErr::Ok;
        }
    }

    MiErr::NotFound
}

/// 翻译器句柄
pub struct MiTranslator {
    inner: FQMTranslator,
}

#[no_mangle]
pub extern "C" fn mi_translator_create() -> *mut MiTranslator {
    Box::into_raw(Box::new(MiTranslator {
        inner: FQMTranslator::new(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn mi_translator_destroy(t: *mut MiTranslator) {
    if !t.is_null() {
        drop(Box::from_raw(t));
    }
}

#[no_mangle]
pub unsafe extern "C" fn mi_translator_translate(
    t: *const MiTranslator,
    foreign: *const c_char,
    out: *mut c_char,
    out_len: usize,
) -> MiErr {
    if t.is_null() || foreign.is_null() || out.is_null() {
        return MiErr::Invalid;
    }

    let foreign_str = match CStr::from_ptr(foreign).to_str() {
        Ok(s) => s,
        Err(_) => return MiErr::Invalid,
    };

    let translator = &(*t).inner;

    if let Some(result) = translator.translate(foreign_str) {
        let result_bytes = result.as_bytes();
        let len = std::cmp::min(result_bytes.len(), out_len - 1);
        let out_slice = slice::from_raw_parts_mut(out as *mut u8, len);
        out_slice.copy_from_slice(&result_bytes[..len]);
        *out.add(len) = 0;
        MiErr::Ok
    } else {
        MiErr::NotFound
    }
}

/// GCD 公约数
#[no_mangle]
pub extern "C" fn mi_gcd(a: u8, b: u8) -> u8 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/// LCM 公倍数
#[no_mangle]
pub extern "C" fn mi_lcm(a: u8, b: u8) -> u16 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a as u16 * b as u16) / mi_gcd(a, b) as u16
}
