// ============================================================================
// 囡囝共生 · MetaIME Rust 核心 — 入口
// ============================================================================
//
// 当前层号: 0 (核心层,非七层编码驱动中的一层,是它们的载体)
// 当前声母: he (核)
// 当前韵母组合: he + xin = 核心 (编码解析+规则判定+分类器+上传登记 四核同体)
// 折叠状态: 未折叠
// 下一步操作: JNI 容器→encode_parser→rules→classifier→自动输出 / URL 抓取→上传登记
//
// 锚定法则:
//   0 = 刘楚恬 (底层隐藏, 不可见)
//   1 = 我 (壳层可见, 不动)
//
// 容器守恒: 从 Kotlin 来的密封容器在此解封, 经过完整管线后回传
//
// 七层管线 (Rust 侧三层 + 上传登记):
//   JNI容器 → 解封 → EncodeParser(第2层) → RuleEngine(第3层) → ShengYunEngine(第4层) → 自动输出(第5层)
//                                    ↓
//                           URL 检测 → 内容抓取(第6层) → 上传登记(第7层)
// ============================================================================

pub mod encode_parser;
pub mod rules;
pub mod classifier;
pub mod upload_channel;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jint, jstring};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use encode_parser::EncodeParser;
use rules::RuleEngine;
use classifier::ShengYunEngine;
use upload_channel::UploadRegistry;

// ── 第2层: 编码解析器 ──
static PARSER: Lazy<Mutex<EncodeParser>> = Lazy::new(|| {
    Mutex::new(EncodeParser::new())
});

// ── 第3层: 规则引擎 ──
static ENGINE: Lazy<Mutex<RuleEngine>> = Lazy::new(|| {
    Mutex::new(RuleEngine::new())
});

// ── 第4层: 声母-韵母分类器 ──
static SHENG_YUN: Lazy<Mutex<ShengYunEngine>> = Lazy::new(|| {
    Mutex::new(ShengYunEngine::new())
});

// ── 第5层: 自动输出判定状态 ──
static AUTO_OUTPUT_BUF: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::new())
});

// ── 第7层: 上传登记簿 ──
static UPLOAD_REG: Lazy<Mutex<UploadRegistry>> = Lazy::new(|| {
    Mutex::new(UploadRegistry::new())
});

// ── 候字缓冲区 ──
static CANDIDATE_BUF: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::new())
});

// ============================================================
// JNI 入口
// ============================================================

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    if let Ok(mut p) = PARSER.lock() { p.reset(); } else { return -10; }
    if let Ok(mut sy) = SHENG_YUN.lock() { sy.reload(); } else { return -11; }
    if let Ok(mut eng) = ENGINE.lock() { eng.reset(); } else { return -12; }
    if let Ok(mut buf) = CANDIDATE_BUF.lock() { buf.clear(); }
    if let Ok(mut ao) = AUTO_OUTPUT_BUF.lock() { ao.clear(); } else { return -13; }
    if let Ok(mut reg) = UPLOAD_REG.lock() { reg.reset(); } else { return -14; }
    0
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeSubmitContainer(
    _env: JNIEnv,
    _class: JClass,
    sealed: jlong,
) -> jint {
    let code_point = ((sealed >> 32) & 0xFFFFFFFF) as u32;
    let sheng_mu = ((sealed >> 24) & 0xFF) as u8;
    let yun_mu = ((sealed >> 16) & 0xFF) as u8;
    let layer = ((sealed >> 8) & 0xFF) as u8;

    // ── 第2层: 编码解析 ──
    let syllable_hit: Option<(u8, u8)> = {
        let mut p = match PARSER.lock() {
            Ok(g) => g,
            Err(_) => return -1,
        };
        match p.ingest(code_point, sheng_mu, yun_mu) {
            Some(unit) => Some((unit.sheng_idx, unit.yun_idx)),
            None => {
                if code_point < 128 {
                    Some((sheng_mu, yun_mu))
                } else {
                    None
                }
            }
        }
    };

    // ── 第3层 + 第4层: 规则 + 候选 ──
    if let Some((sm, ym)) = syllable_hit {
        let eng_verdict = {
            let mut eng = match ENGINE.lock() {
                Ok(g) => g,
                Err(_) => return -2,
            };
            eng.ingest(code_point, sm, ym, layer)
        };

        let cands = {
            let sy = match SHENG_YUN.lock() {
                Ok(g) => g,
                Err(_) => return -3,
            };
            sy.generate_candidates(code_point, sm, ym)
        };

        if !cands.is_empty() {
            let mut buf = match CANDIDATE_BUF.lock() {
                Ok(g) => g,
                Err(_) => return -4,
            };
            buf.clear();
            for (i, c) in cands.iter().enumerate() {
                if i > 0 { buf.push('\0'); }
                buf.push_str(c);
            }
            if eng_verdict && !cands.is_empty() { 1 } else { 0 }
        } else {
            0
        }
    } else {
        // 无音节产出 — 字母自身作为候字
        if code_point < 128 && code_point > 32 {
            if let Some(c) = char::from_u32(code_point) {
                let mut buf = match CANDIDATE_BUF.lock() {
                    Ok(g) => g,
                    Err(_) => return -5,
                };
                buf.clear();
                buf.push(c);
            }
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeTakeCandidates<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let buf = CANDIDATE_BUF.lock()
        .map(|g| g.clone())
        .unwrap_or_default();
    let out = env.new_string(buf).expect("take_candidates failed");
    out.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeFullPinyin<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let pinyin = PARSER.lock()
        .map(|p| p.full_pinyin())
        .unwrap_or_default();
    let out = env.new_string(pinyin).expect("full_pinyin failed");
    out.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeSyllableSeq<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let seq = PARSER.lock()
        .map(|p| {
            let mut s = String::from("[");
            for (i, syll) in p.syllable_seq().iter().enumerate() {
                if i > 0 { s.push(','); }
                s.push('"');
                s.push_str(syll);
                s.push('"');
            }
            s.push(']');
            s
        })
        .unwrap_or_else(|_| String::from("[]"));
    let out = env.new_string(seq).expect("syllable_seq failed");
    out.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeEngineStatus<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let eng = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    let sy = SHENG_YUN.lock().unwrap_or_else(|e| e.into_inner());
    let parser = PARSER.lock().unwrap_or_else(|e| e.into_inner());
    let desc = format!(
        "[MetaIME v2.0-7layer] parser=[{}] rules=[{}] class=[{}] layer={} total={}",
        parser.state_desc(),
        eng.state_desc(),
        sy.state_desc(),
        eng.active_layer(),
        eng.container_count()
    );
    let out = env.new_string(desc).expect("engine_status failed");
    out.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeCurrentLayer(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    ENGINE.lock()
        .map(|eng| eng.active_layer())
        .unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeOnCandidateSelected(
    mut env: JNIEnv,
    _class: JClass,
    candidate: JString,
) {
    let cand: String = env.get_string(&candidate)
        .expect("read candidate failed")
        .into();
    if let Ok(mut eng) = ENGINE.lock() {
        eng.on_candidate_chosen(&cand);
    }
    if let Ok(mut p) = PARSER.lock() {
        // 音节环也清空, 新一轮
        p.reset();
    }
}

// ============================================================
// 第5层: 自动输出推入缓冲
// ============================================================

/// 当 Kotlin 侧决定自动输出某个候字时, 将其推入 AUTO_OUTPUT_BUF
/// 供 nativeAutoOutputText 取用 (或直接作为回传)
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeSubmitForAutoOutput(
    mut env: JNIEnv,
    _class: JClass,
    candidate: JString,
) -> jint {
    let cand: String = match env.get_string(&candidate) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    if let Ok(mut buf) = AUTO_OUTPUT_BUF.lock() {
        buf.clear();
        buf.push_str(&cand);
        0
    } else {
        -2
    }
}

/// 重置自动输出缓冲
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeResetAutoOutput(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    if let Ok(mut buf) = AUTO_OUTPUT_BUF.lock() {
        buf.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeOnBackspace(
    _env: JNIEnv,
    _class: JClass,
) {
    if let Ok(mut eng) = ENGINE.lock() {
        eng.on_backspace();
    }
    if let Ok(mut p) = PARSER.lock() {
        p.pop_syllable();
    }
}

// ============================================================
// 第5层 JNI: 自动输出判定
// ============================================================

/// 通知 Rust 核心: 某个音节已被完整产出, 可做自动输出判定
/// 返回 1 = 应当立即自动输出, 0 = 不输出, 等待下一按键
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeAutoCommitCheck(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    // 自动输出判定: 若规则引擎最近一次判定为 AutoOutput, 返回 1
    if let Ok(eng) = ENGINE.lock() {
        // 简化的判定: 当音节序列中最近的 verdict 非 Pass 时触发
        if eng.active_layer() >= 3 {
            return 1;
        }
    }
    0
}

/// 取 Rust 核心准备好的自动输出文本 (由第5层判决后推入)
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeAutoOutputText<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let text = AUTO_OUTPUT_BUF.lock()
        .map(|g| g.clone())
        .unwrap_or_default();
    let out = env.new_string(text).expect("auto_output_text failed");
    out.into_raw()
}

// ============================================================
// 第6层 JNI: URL 抓取触发
// ============================================================

/// 通知核心: 检测到 URL 模式, 推入后台抓取队列
/// 返回: 推入后的队列长度
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeOnUrlDetected(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jint {
    let raw: String = match env.get_string(&url) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    // URL 推入上传登记簿 (第7层将推送到三通道)
    if let Ok(mut reg) = UPLOAD_REG.lock() {
        reg.register(&raw);
        return reg.len() as jint;
    }
    -1
}

// ============================================================
// 第7层 JNI: 多通道上传登记
// ============================================================

/// 触发上传: 将所有 Pending 条目推送到三通道
/// 返回: 推送的条目数
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeUploadDispatch(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    if let Ok(mut reg) = UPLOAD_REG.lock() {
        return reg.dispatch_all() as jint;
    }
    -1
}

/// 取上传登记簿 JSON 状态 (供调试面板展示)
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeUploadStatus<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let json = UPLOAD_REG.lock()
        .map(|reg| reg.status_json())
        .unwrap_or_else(|_| String::from("[]"));
    let out = env.new_string(json).expect("upload_status failed");
    out.into_raw()
}

/// 取上传统计: 以逗号分隔 (总数,已送达,失败)
#[no_mangle]
pub extern "C" fn Java_com_njsg_parasite_NativeBridge_nativeUploadStats<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
) -> jstring {
    let (total, landed, failed) = UPLOAD_REG.lock()
        .map(|reg| reg.stats())
        .unwrap_or((0, 0, 0));
    let desc = format!("{},{},{}", total, landed, failed);
    let out = env.new_string(desc).expect("upload_stats failed");
    out.into_raw()
}

// ============================================================
// 单元测试 (cargo test --no-run 提供离线静态验证)
// ============================================================
#[cfg(test)]
mod tests {
    use super::encode_parser::EncodeParser;

    #[test]
    fn test_parser_basic() {
        let mut p = EncodeParser::new();
        // 键入 'g' → 声母 idx=8
        p.ingest(b'g' as u32, 8, 255);
        // 键入 'u' → 韵母 idx=4
        let unit = p.ingest(b'u' as u32, 255, 4);
        assert!(unit.is_some());
        let u = unit.unwrap();
        assert_eq!(u.sheng_idx, 8); // g
        assert_eq!(u.yun_idx, 4);   // u
        assert_eq!(u.pinyin, "gu");
    }

    #[test]
    fn test_parser_sequence() {
        let mut p = EncodeParser::new();
        p.ingest(b'n' as u32, 6, 255);   // n
        p.ingest(b'i' as u32, 255, 3);    // i → "ni"
        p.ingest(b'h' as u32, 10, 255);  // h
        p.ingest(b'a' as u32, 255, 0);    // a → "ha"
        p.ingest(b'o' as u32, 255, 1);    // o → "hao"? no, pending_yun: [0,1]
        // 实际上 'a' 又推入韵母, 与声母 'h' 组合成 "ha"
        // 然后 'o' 等待下一声母
        let syll = p.syllable_seq();
        assert!(syll.len() >= 2); // "ni" + "ha"
    }

    #[test]
    fn test_parser_reset() {
        let mut p = EncodeParser::new();
        p.ingest(b't' as u32, 5, 255);
        p.reset();
        assert!(p.syllable_seq().is_empty());
    }
}
