// ╔══════════════════════════════════════════════════════════════════╗
// ║  元输入法 MetaIME · JNI 桥接层                                    ║
// ║  Rust ↔ Kotlin 双向绑定 · 自动生成 JNI 函数                       ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝

use jni::objects::JClass;
use jni::sys::{jbyte, jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use mqf::metaime::{self, MetaImeState};
use mqf::metaime_flow::FlowState;
use std::ffi::CStr;
use std::sync::Mutex;

const NATIVE_VERSION: &str = "MetaIME/JNI v0.1.1";

// ── 全局 FlowState (线程安全) ──────────────────────────────────────────

thread_local! {
    static FLOW_STATE: Mutex<Option<FlowState>> = Mutex::new(None);
}

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

// ── 流式输入引擎 JNI 接口 ───────────────────────────────────────────────

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_flowInit(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle == 0 { return; }
    FLOW_STATE.with(|f| {
        let mut guard = f.lock().unwrap();
        *guard = Some(FlowState::new());
    });
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_flowInput(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    ch: jbyte,
) -> jstring {
    if handle == 0 { return std::ptr::null_mut(); }

    let (segs, cur, cands, done) = FLOW_STATE.with(|f| {
        let mut guard = f.lock().unwrap();
        if let Some(ref mut flow) = *guard {
            let r = flow.input_char(ch as u8 as char, handle as *mut MetaImeState);
            (r.segments.clone(), r.current_display.clone(), r.current_candidates.clone(), r.completed)
        } else {
            (Vec::new(), String::new(), Vec::new(), false)
        }
    });

    let json = format!(
        r#"{{"segments":{:?},"current":"{}","candidates":{:?},"completed":{}}}"#,
        segs, cur, cands, done
    );

    env.new_string(&json)
        .map(|j| j.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_flowReset(
    _env: JNIEnv,
    _class: JClass,
    _handle: jlong,
) {
    FLOW_STATE.with(|f| {
        let mut guard = f.lock().unwrap();
        *guard = None;
    });
}

#[no_mangle]
pub extern "system" fn Java_com_manquanca_metaime_MetaIMENative_flowBackspace(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jstring {
    if handle == 0 { return std::ptr::null_mut(); }

    let result = FLOW_STATE.with(|f| {
        let mut guard = f.lock().unwrap();
        if let Some(ref mut flow) = *guard {
            flow.input_char('\x08', handle as *mut MetaImeState)
        } else {
            return String::new();
        }
    });

    let json = format!(
        r#"{{"segments":{:?},"current":"{}","candidates":{:?},"completed":{}}}"#,
        result.segments,
        result.current_display,
        result.current_candidates,
        result.completed
    );

    env.new_string(&json)
        .map(|j| j.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

// main 占位 (仅 cargo check 需要)
fn main() {}
