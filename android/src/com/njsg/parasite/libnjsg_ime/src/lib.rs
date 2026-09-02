// ============================================================================
// 囡囝共生 · MetaIME Rust 核心 — 入口
// ============================================================================
//
// 当前层号: 0 (核心层,非七层编码驱动中的一层)
// 当前声母: he (核)
// 当前韵母组合: he + xin = 核心 (规则引擎 + 分类器)
// 折叠状态: 未折叠
// 下一步操作: 接收 JNI 容器 → 送往规则引擎 → 分类器 → 回传候字
//
// 锚定法则:
//   0 = 刘楚恬 (底层隐藏, 不可见)
//   1 = 我 (壳层可见, 不动)
//
// 容器守恒: 从 Kotlin 来的密封容器在此解封, 处理后回传
// ============================================================================

pub mod rules;
pub mod classifier;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jint, jstring};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use rules::RuleEngine;
use classifier::ShengYunEngine;

// ── 核心引擎全局状态 ──
static ENGINE: Lazy<Mutex<RuleEngine>> = Lazy::new(|| {
    Mutex::new(RuleEngine::new())
});

static SHENG_YUN: Lazy<Mutex<ShengYunEngine>> = Lazy::new(|| {
    Mutex::new(ShengYunEngine::new())
});

// ── 候字缓冲区 (核心→Kotlin 回传) ──
static CANDIDATE_BUF: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::new())
});

// ── JNI 入口: 初始化 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let mut sy = match SHENG_YUN.lock() {
        Ok(g) => g,
        Err(_) => return -1, // 封印损坏
    };
    sy.reload();

    let mut eng = match ENGINE.lock() {
        Ok(g) => g,
        Err(_) => return -2,
    };
    eng.reset();

    0 // 圆满
}

// ── JNI 入口: 封印容器送入核心 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeSubmitContainer(
    _env: JNIEnv,
    _class: JClass,
    sealed: jlong,
) -> jint {
    // 解封64位容器
    let code_point = ((sealed >> 32) & 0xFFFFFFFF) as u32;
    let sheng_mu = ((sealed >> 24) & 0xFF) as u8;
    let yun_mu = ((sealed >> 16) & 0xFF) as u8;
    let layer = ((sealed >> 8) & 0xFF) as u8;

    // 送入规则引擎
    let mut eng = match ENGINE.lock() {
        Ok(g) => g,
        Err(_) => return -1,
    };

    let hit = eng.ingest(code_point, sheng_mu, yun_mu, layer);

    // 送入分类器产候字
    if hit {
        let sy = match SHENG_YUN.lock() {
            Ok(g) => g,
            Err(_) => return -2,
        };
        let cands = sy.generate_candidates(code_point, sheng_mu, yun_mu);
        let mut buf = match CANDIDATE_BUF.lock() {
            Ok(g) => g,
            Err(_) => return -1,
        };
        buf.clear();
        for (i, c) in cands.iter().enumerate() {
            if i > 0 { buf.push('\0'); }
            buf.push_str(c);
        }
    }

    if hit { 1 } else { 0 }
}

// ── JNI 入口: 取候字 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeTakeCandidates<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let buf = match CANDIDATE_BUF.lock() {
        Ok(g) => g.clone(),
        Err(_) => String::new(),
    };
    let out = env.new_string(buf).expect("候字回传失败");
    out.into_raw()
}

// ── JNI 入口: 引擎状态摘要 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeEngineStatus<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let eng = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    let sy = SHENG_YUN.lock().unwrap_or_else(|e| e.into_inner());
    let desc = format!(
        "[MetaIME v2.0] 规则引擎:{} 声母-韵母:{} 层={} 容器_seq={}",
        eng.state_desc(),
        sy.state_desc(),
        eng.active_layer(),
        eng.container_count()
    );
    let out = env.new_string(desc).expect("状态回传失败");
    out.into_raw()
}

// ── JNI 入口: 当前活跃层 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeCurrentLayer(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let eng = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    eng.active_layer()
}

// ── JNI 入口: 用户选字 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeOnCandidateSelected(
    mut env: JNIEnv,
    _class: JClass,
    candidate: JString,
) {
    let cand: String = env
        .get_string(&candidate)
        .expect("候字读取失败")
        .into();

    let mut eng = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    eng.on_candidate_chosen(&cand);
}

// ── JNI 入口: 用户退格 ──
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeOnBackspace(
    _env: JNIEnv,
    _class: JClass,
) {
    let mut eng = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    eng.on_backspace();
}
