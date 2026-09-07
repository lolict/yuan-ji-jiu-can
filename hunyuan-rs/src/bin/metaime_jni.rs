// ╔══════════════════════════════════════════════════════════════════╗
// ║  元输入法 MetaIME · JNI 桥接层                                    ║
// ║  Rust ↔ Kotlin 双向绑定 · 自动生成 JNI 函数                       ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝

use jni::objects::JClass;
use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use mqf::metaime::{self, MetaImeState};
use std::ffi::CStr;

const NATIVE_VERSION: &str = "MetaIME/JNI v0.1.0";

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_init(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    metaime::metaime_init() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_free(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        metaime::metaime_free(handle as *mut MetaImeState);
    }
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_query(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    shengmu: u8,
    yunmu: u8,
    tone: u8,
) -> jstring {
    if handle == 0 {
        return std::ptr::null_mut();
    }
    let state = handle as *mut MetaImeState;
    let cstr_ptr = metaime::metaime_query_cstr(state, shengmu, yunmu, tone);
    if cstr_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let result = match unsafe { CStr::from_ptr(cstr_ptr) }.to_str() {
        Ok(s) => env.new_string(s).map(|j| j.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    };
    metaime::metaime_string_free(cstr_ptr);
    result
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_queryBytes(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    shengmu: u8,
    yunmu: u8,
    tone: u8,
) -> jbyteArray {
    if handle == 0 {
        return std::ptr::null_mut();
    }
    let state = handle as *mut MetaImeState;
    let cstr_ptr = metaime::metaime_query_cstr(state, shengmu, yunmu, tone);
    if cstr_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let bytes = unsafe { CStr::from_ptr(cstr_ptr) }.to_bytes().to_vec();
    let result = match env.byte_array_from_slice(&bytes) {
        Ok(arr) => arr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    };
    metaime::metaime_string_free(cstr_ptr);
    result
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_getVersion(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string(NATIVE_VERSION)
        .map(|j| j.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_getShengmuCount(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    24
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_getYunmuCount(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    35
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_getMaxTone(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    4
}

// JNI_OnLoad: Android 动态库加载入口
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    _vm: jni::JavaVM,
    _reserved: *mut std::os::raw::c_void,
) -> jint {
    jni::sys::JNI_VERSION_1_6
}

// main 占位 (仅 cargo check 需要)
fn main() {}
